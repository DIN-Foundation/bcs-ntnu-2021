pub fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        CMD::Init{ path } => init(&path),
        CMD::Doc{ path } => doc(&path),
        CMD::Did{ path } => did(&path),
        CMD::Connect{ path, name, did } => connect(&path, &name, &did),
        CMD::Send{ path, name, message } => send(&path, &name, &message),
        CMD::Receive{ path, encrypted_message } => receive(&path, &encrypted_message),
        CMD::Help => help()
    }
}

/**
 * new - Creates a public/private key-pair if does not already exists, effectively creating a new chat.
 */ 
fn init(path: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Create empty folders
    if !std::fs::metadata(root_path(path)).is_ok() {
        std::fs::create_dir_all(path).unwrap_or_default();
    }

    if !std::fs::metadata(names_path(path)).is_ok() {
        std::fs::create_dir_all(names_path(path)).unwrap_or_default();
    }

    if !std::fs::metadata(dids_path(path)).is_ok() {
        std::fs::create_dir_all(dids_path(path)).unwrap_or_default();
    }

    if !std::fs::metadata(seed_path(path)).is_ok() {
        // 2. Generate seed
        let mut csprng = rand_core::OsRng{};
        let seed = ed25519_dalek::SecretKey::generate(&mut csprng);
        let seed_bytes = seed.as_bytes();

        // 3. Write seed to file
        let mut file = std::fs::File::create(seed_path(path)).unwrap();
        file.write(seed_bytes).unwrap();
        
        Ok(format!("{} is ready", path))
    } else {
        Ok(format!("{} already exists", path))
    }
}

fn doc(path: &str) -> Result<String, std::io::Error> {
    use did_key::DIDCore;
    use did_key::KeyMaterial;
    
    // 1. Read seed from file
    let seed = std::fs::read(seed_path(path)).unwrap();

    // 2. Transform seed to a did-document
    let public_private_keypair = did_key::Ed25519KeyPair::from_seed(&seed);
    let public_only_keypair = did_key::from_existing_key::<did_key::Ed25519KeyPair>(&public_private_keypair.public_key_bytes(), None);
    let doc = public_only_keypair.get_did_document(did_key::CONFIG_LD_PUBLIC);

    // 3. Serialize did-document to json
    let did_document = serde_json::to_string_pretty(&doc).unwrap();
    Ok(format!("{}", did_document))
}

fn did(path: &str) -> Result<String, std::io::Error> {
    use did_key::DIDCore;

    // 1. Read seed from file
    let seed = std::fs::read(seed_path(path)).unwrap();

    // 2. Transform seed to a did
    let keypair = did_key::Ed25519KeyPair::from_seed(&seed);
    let diddoc: did_key::Document = keypair.get_did_document(did_key::CONFIG_LD_PUBLIC);
    let did = diddoc.id;

    // 3. Print did
    Ok(format!("{}", did))
}

fn connect(path: &str, name: &str, did: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 2. Create 'name' -> 'did' mapping
    let mut file = std::fs::File::create(name_path(path, name)).unwrap();
    file.write(did.as_bytes()).unwrap();

    // 3. Create 'did' to 'name' mapping
    let mut file = std::fs::File::create(did_path(path, did)).unwrap();
    file.write(name.as_bytes()).unwrap();

    Ok(format!("{}\n{}", name_path(path, name), did_path(path, did)))
}

fn send(path: &str, name: &str, message: &str) -> Result<String, std::io::Error> {
    use did_key::Ecdh;
    use did_key::KeyMaterial;
    use did_key::DIDCore;

    // 1. Read from and to x25519 keys from file
    let from_seed = std::fs::read(seed_path(path)).unwrap();
    let from_key = did_key::Ed25519KeyPair::from_seed(&from_seed);
    let from_did = from_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;
    let from_key = from_key.get_x25519();
    
    let to_did = std::fs::read_to_string(name_path(path, name)).unwrap();
    let to_key = did_key::resolve(&to_did).unwrap();
    let to_key = did_key::Ed25519KeyPair::from_public_key(&to_key.public_key_bytes());
    let to_key = to_key.get_x25519();

    // 2. Generate elliptic-curve-diffie-hellmann (ecdh) shared secret
    let shared_secret = from_key.key_exchange(&to_key);

    // 3. Construct didcomm message
    let to_vec = vec![&to_did[..]];
    
    let message = didcomm_rs::Message::new()
        .from(&from_did)
        .to(&to_vec[..])
        .timed(Some(3600))
        .body(message.as_bytes())
        .as_jwe(&didcomm_rs::crypto::CryptoAlgorithm::XC20P);

    // 4. Seal/encrypt message using shared secret
    let encrypted_message = message
        .seal(&shared_secret)
        .unwrap();

    Ok(format!("{}", &encrypted_message))
}

fn receive(path: &str, encrypted_message: &str) -> Result<String, std::io::Error> {
    use did_key::Ecdh;
    use did_key::KeyMaterial;

    // 1. Read "to"-key from file
    let to_seed = std::fs::read(seed_path(path)).unwrap();
    let to_key = did_key::Ed25519KeyPair::from_seed(&to_seed);
    let to_key = to_key.get_x25519();

    // 2. Read "from"-key from "dcem" header.
    let jwe: didcomm_rs::Jwe = serde_json::from_str(encrypted_message).unwrap();
    let from_did = jwe.from().as_ref().unwrap();
    let from_key = did_key::resolve(&from_did).unwrap();
    let from_key = did_key::Ed25519KeyPair::from_public_key(&from_key.public_key_bytes());
    let from_key = from_key.get_x25519();

    // 3. Generate elliptic-curve-diffie-hellmann (ecdh) shared secret
    let shared_secret = to_key.key_exchange(&from_key);

    // 4. Decrypt message
    let received = didcomm_rs::Message::receive(encrypted_message, Some(&shared_secret), None);
    let received = received.unwrap(); // @unwrap!
    let received = String::from_utf8(received.body).unwrap(); // @unwrap!

    // 5. Map did to name
    let from_name = std::fs::read_to_string(did_path(path, from_did))
        .unwrap_or(from_did.clone());

    Ok(format!("{} > {}", from_name, received))
}

fn help() -> Result<String, std::io::Error> {
    Ok(String::from("
    Usage:
        didchat <path> init
        didchat <path> doc    
        didchat <path> did    

        didchat <path> connect <name> <did>

        didchat <path> send    <name> <message>      -->  <encrypted message>
        didchat <path> receive <encrypted message>   -->  <name> > <message>

    Example - Talk to self:
        didchat . init
        didchat . connect self $(didchat . did)
        didchat . receive $(didchat . send self \"Hello self!\")

    Example - Talk to peer:
        didchat jonas init
        didchat snorre init
        
        didchat snorre connect jonas $(didchat jonas did)
        didchat jonas connect snorre $(didchat snorre did)

        didchat jonas receive $(didchat snorre send jonas \"Hello Jonas. How are you?\")
        didchat snorre receive $(didchat jonas send snorre \"Hi Snorre:) I have seen better days.\")
"))
}


#[derive(Debug)]
enum CMD {
    Init{ path: String },
    Doc{ path: String },
    Did{ path: String },
    Connect{ path: String, name: String, did: String },
    Send{ path: String, name: String, message: String },
    Receive{ path: String, encrypted_message: String },
    Help
}

pub struct Config {
    cmd: CMD,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let valid_cmds = vec!["help", "did", "doc", "init", "connect", "send", "receive"];
        let default_cmd = String::from("help");
        
        let path = args.get(1).unwrap_or(&default_cmd);
        let cmd = args.get(2).unwrap_or(&default_cmd);

        let cmd = if args.len() < 3 {
            default_cmd.clone()
        } else if valid_cmds.contains(&&cmd[..]) {
            cmd.clone()
        } else  {
            default_cmd.clone()
        };

        let path = path.clone();

        let cmd: CMD = match &cmd[..] {
            "did" => {
                CMD::Did{ path }
            },
            "doc" => {
                CMD::Doc{ path }
            },
            "init" => {
                CMD::Init{ path }
            },
            "connect" => {
                let name = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let did = (match args.get(4) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();
                
                CMD::Connect{ path, name, did }
            },
            "send" => {
                let name = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let message = (match args.get(4) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Send{ path, name, message }
            },
            "receive" => {
                let encrypted_message = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Receive{ path, encrypted_message }
            },
            "help" => CMD::Help,
            &_ => CMD::Help,
        };

        Ok(Config { cmd })
    }
}

fn root_path(path: &str) -> String {
    format!("{}/.didchat", path)
}

fn seed_path(path: &str) -> String {
    format!("{}/.didchat/seed", path)
}

fn names_path(path: &str) -> String {
    format!("{}/.didchat/names", path)
}

fn name_path(path: &str, name: &str) -> String {
    format!("{}/.didchat/names/{}", path, name)
}

fn dids_path(path: &str) -> String {
    format!("{}/.didchat/dids", path)
}

fn did_path(path: &str, did: &str) -> String {
    format!("{}/.didchat/dids/{}", path, did)
}
