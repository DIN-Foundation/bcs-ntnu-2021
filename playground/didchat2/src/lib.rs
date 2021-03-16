pub fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        CMD::Init{ agent } => init(&agent),
        CMD::Doc{ agent } => doc(&agent),
        CMD::Did{ agent } => did(&agent),
        CMD::Connect{ agent, name, did } => connect(&agent, &name, &did),
        CMD::Send{ agent, name, message } => send(&agent, &name, &message),
        CMD::Receive{ agent, didcomm_encrypted_message } => receive(&agent, &didcomm_encrypted_message),
        CMD::Help => help()
    }
}

/**
 * new - Creates a public/private key-pair if does not already exists, effectively creating a new chat.
 */ 
fn init(agent: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    if !std::fs::metadata(root_path(agent)).is_ok() {
        std::fs::create_dir_all(root_path(agent)).unwrap_or_default();
    }

    if !std::fs::metadata(names_path(agent)).is_ok() {
        std::fs::create_dir_all(names_path(agent)).unwrap_or_default();
    }

    if !std::fs::metadata(seed_path(agent)).is_ok() {
        // 1. Generate seed
        let mut csprng = rand_core::OsRng{};
        let seed = ed25519_dalek::SecretKey::generate(&mut csprng);
        let seed_bytes = seed.as_bytes();

        // 2. Write seed to file
        let mut file = std::fs::File::create(seed_path(agent)).unwrap();
        file.write(seed_bytes).unwrap();
        
        // 3. Connect to self
        let did = did(agent).unwrap();
        let _ = connect(agent, agent, &did);

        Ok(format!("{} is ready", root_path(agent)))
    } else {
        Ok(format!("{} already exists", root_path(agent)))
    }
}

fn doc(agent: &str) -> Result<String, std::io::Error> {
    use did_key::DIDCore;
    use did_key::KeyMaterial;
    
    // 1. Read seed from file
    let seed = std::fs::read(seed_path(agent)).unwrap();

    // 2. Transform seed to a did-document
    let public_private_keypair = did_key::Ed25519KeyPair::from_seed(&seed);
    let public_only_keypair = did_key::from_existing_key::<did_key::Ed25519KeyPair>(&public_private_keypair.public_key_bytes(), None);
    let doc = public_only_keypair.get_did_document(did_key::CONFIG_LD_PUBLIC);

    // 3. Serialize did-document to json
    let did_document = serde_json::to_string_pretty(&doc).unwrap();
    Ok(format!("{}", did_document))
}

fn did(agent: &str) -> Result<String, std::io::Error> {
    use did_key::DIDCore;

    // 1. Read seed from file
    let seed = std::fs::read(seed_path(agent)).unwrap();

    // 2. Transform seed to a did
    let keypair = did_key::Ed25519KeyPair::from_seed(&seed);
    let diddoc: did_key::Document = keypair.get_did_document(did_key::CONFIG_LD_PUBLIC);
    let did = diddoc.id;

    // 3. Print did
    Ok(format!("{}", did))
}

fn connect(agent: &str, name: &str, did: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    // 1. Ensure folders exists
    if !std::fs::metadata(names_path(agent)).is_ok() {
        std::fs::create_dir_all(names_path(agent)).unwrap_or_default();
    }

    if !std::fs::metadata(dids_path(agent)).is_ok() {
        std::fs::create_dir_all(dids_path(agent)).unwrap_or_default();
    }

    // 2. Create 'name' -> 'did' mapping
    let mut file = std::fs::File::create(name_path(agent, name)).unwrap();
    file.write(did.as_bytes()).unwrap();

    // 3. Create 'did' to 'name' mapping
    let mut file = std::fs::File::create(did_path(agent, did)).unwrap();
    file.write(name.as_bytes()).unwrap();

    Ok(format!("{}\n{}", name_path(agent, name), did_path(agent, did)))
}

fn send(agent: &str, name: &str, message: &str) -> Result<String, std::io::Error> {
    use did_key::Ecdh;
    use did_key::KeyMaterial;
    use did_key::DIDCore;

    // 1. Read from and to x25519 keys from file
    let from_seed = std::fs::read(seed_path(agent)).unwrap();
    let from_key = did_key::Ed25519KeyPair::from_seed(&from_seed);
    let from_did = from_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;
    let from_key = from_key.get_x25519();
    
    let to_did = std::fs::read_to_string(name_path(agent, name)).unwrap();
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
    let didcomm_encrypted_message = message
        .seal(&shared_secret)
        .unwrap();

    Ok(format!("{}", &didcomm_encrypted_message))
}

fn receive(agent: &str, didcomm_encrypted_message: &str) -> Result<String, std::io::Error> {
    use did_key::Ecdh;
    use did_key::KeyMaterial;

    // 1. Read "to"-key from file
    let to_seed = std::fs::read(seed_path(agent)).unwrap();
    let to_key = did_key::Ed25519KeyPair::from_seed(&to_seed);
    let to_key = to_key.get_x25519();

    // 2. Read "from"-key from "dcem" header.
    let jwe: didcomm_rs::Jwe = serde_json::from_str(didcomm_encrypted_message).unwrap();
    let from_did = jwe.from().as_ref().unwrap();
    let from_key = did_key::resolve(&from_did).unwrap();
    let from_key = did_key::Ed25519KeyPair::from_public_key(&from_key.public_key_bytes());
    let from_key = from_key.get_x25519();

    // 3. Generate elliptic-curve-diffie-hellmann (ecdh) shared secret
    let shared_secret = to_key.key_exchange(&from_key);

    // 4. Decrypt message
    let received = didcomm_rs::Message::receive(didcomm_encrypted_message, Some(&shared_secret), None);
    let received = received.unwrap(); // @unwrap!
    let received = String::from_utf8(received.body).unwrap(); // @unwrap!

    // 5. Map did to name
    let from_name = std::fs::read_to_string(did_path(agent, from_did)).unwrap();

    Ok(format!("{}: {}", from_name, received))
}

fn help() -> Result<String, std::io::Error> {
    Ok(String::from("
    Usage: 
        didchat init    <me>
        didchat doc     <me>
        didchat did     <me>

        didchat connect <me>  <other> <other did>

        didchat send    <me>  <other> <message>        -->  <encrypted message>
        didchat receive <me>  <encrypted message>      -->  <other>: <message>
"))
}


#[derive(Debug)]
enum CMD {
    Init{ agent: String },
    Doc{ agent: String },
    Did{ agent: String },
    Connect{ agent: String, name: String, did: String },
    Send{ agent: String, name: String, message: String },
    Receive{ agent: String, didcomm_encrypted_message: String },
    Help
}

pub struct Config {
    cmd: CMD,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let valid_cmds = vec!["help", "did", "doc", "init", "connect", "send", "receive"];
        let default_cmd = String::from("help");
        let cmd = args.get(1).unwrap_or(&default_cmd);

        let cmd = if valid_cmds.contains(&&cmd[..]) {
            cmd.clone()
        } else  {
            default_cmd.clone()
        };

        let cmd: CMD = match &cmd[..] {
            "did" => {
                let agent = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Did{ agent }
            },
            "doc" => {
                let agent = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Doc{ agent }
            },
            "init" => {
                let agent = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Init{ agent }
            },
            "connect" => {
                let agent = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let name = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let did = (match args.get(4) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();
                
                CMD::Connect{ agent, name, did }
            },
            "send" => {
                let agent = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let name = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let message = (match args.get(4) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Send{ agent, name, message }
            },
            "receive" => {
                let agent = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let didcomm_encrypted_message = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Receive{ agent, didcomm_encrypted_message }
            },
            "help" => CMD::Help,
            &_ => CMD::Help,
        };

        Ok(Config { cmd })
    }
}

fn root_path(agent: &str) -> String {
    format!(".didchat.{}", agent)
}

fn seed_path(agent: &str) -> String {
    format!(".didchat.{}/seed", agent)
}

fn names_path(agent: &str) -> String {
    format!(".didchat.{}/names", agent)
}

fn name_path(agent: &str, name: &str) -> String {
    format!(".didchat.{}/names/{}", agent, name)
}

fn dids_path(agent: &str) -> String {
    format!(".didchat.{}/dids", agent)
}

fn did_path(agent: &str, did: &str) -> String {
    format!(".didchat.{}/dids/{}", agent, did)
}
