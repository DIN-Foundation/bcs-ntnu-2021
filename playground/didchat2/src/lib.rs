pub fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        CMD::Init => init(),
        CMD::Doc => doc(),
        CMD::Did => did(),
        CMD::Add{ name, did } => add(&name, &did),
        CMD::Send{ to, message } => send(&to, &message),
        CMD::Receive{ didcomm_message } => receive(&didcomm_message),
        CMD::Help => help()
    }
}

/**
 * new - Creates a public/private key-pair if does not already exists, effectively creating a new chat.
 */ 
fn init() -> Result<String, std::io::Error> {
    use std::io::Write;

    if !std::fs::metadata(".didchat/").is_ok() {
        std::fs::create_dir_all(".didchat/").unwrap_or_default();
    }

    let seed_path = format!(".didchat/seed");

    if !std::fs::metadata(&seed_path).is_ok() {
        // 1. Generate seed
        let mut csprng = rand_core::OsRng{};
        let seed = ed25519_dalek::SecretKey::generate(&mut csprng);
        let seed_bytes = seed.as_bytes();

        // 2. Write seed to file
        let mut file = std::fs::File::create(seed_path).unwrap();
        file.write(seed_bytes).unwrap();
        
        Ok(format!("Didchat is ready."))
    } else {
        Ok(format!("Existing user already signed in."))
    }
}

fn doc() -> Result<String, std::io::Error> {
    use did_key::DIDCore;
    use did_key::KeyMaterial;
    
    // 1. Read seed from file
    let seed = std::fs::read(".didchat/seed").unwrap();

    // 2. Transform seed to a did-document
    let public_private_keypair = did_key::Ed25519KeyPair::from_seed(&seed);
    let public_only_keypair = did_key::from_existing_key::<did_key::Ed25519KeyPair>(&public_private_keypair.public_key_bytes(), None);
    let doc = public_only_keypair.get_did_document(did_key::CONFIG_LD_PUBLIC);

    // 3. Serialize did-document to json
    let json = serde_json::to_string_pretty(&doc).unwrap();
    Ok(format!("{}", json))
}

fn did() -> Result<String, std::io::Error> {
    use did_key::DIDCore;

    // 1. Read seed from file
    let seed = std::fs::read(".didchat/seed").unwrap();

    // 2. Transform seed to a did
    let keypair = did_key::Ed25519KeyPair::from_seed(&seed);

    let diddoc: did_key::Document = keypair.get_did_document(did_key::CONFIG_LD_PUBLIC);
    let did = diddoc.id;

    // 3. Print did
    Ok(format!("{}", did))
}

fn add(alias: &str, did: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    if !std::fs::metadata(".didchat/dids/").is_ok() {
        std::fs::create_dir_all(".didchat/dids/").unwrap_or_default();
    }

    let friend_path = format!(".didchat/dids/{}.did", alias);
    let mut file = std::fs::File::create(friend_path).unwrap();
    file.write(did.as_bytes()).unwrap();

    Ok(format!(".didchat/dids/{}.did", alias))
}

fn send(to: &str, message: &str) -> Result<String, std::io::Error> {
    use did_key::Ecdh;
    use did_key::KeyMaterial;
    use did_key::DIDCore;

    // 1. Read from and to x25519 keys from file
    let from_seed = std::fs::read(".didchat/seed").unwrap(); // @unwrap!!
    let from_key = did_key::Ed25519KeyPair::from_seed(&from_seed);
    let from_did = from_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;
    let from_key = from_key.get_x25519();
    
    let to_did = std::fs::read_to_string(format!(".didchat/dids/{}.did", to)).unwrap(); // @unwrap!!
    let to_key = did_key::resolve(&to_did).unwrap(); // @unwrap!!
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
    let sealed_message = message
        .seal(&shared_secret)
        .unwrap(); // @unwrap!!

    Ok(format!("{:?}", &sealed_message))
}

fn receive(didcomm_message: &str) -> Result<String, std::io::Error> {
    use did_key::Ecdh;
    use did_key::KeyMaterial;

    // 1. Read "to"-key from file
    let to_seed = std::fs::read(".didchat/seed").unwrap(); // @unwrap!!
    let to_key = did_key::Ed25519KeyPair::from_seed(&to_seed);
    let to_key = to_key.get_x25519();

    // 2. Read "from"-key from did file
    let jwe: didcomm_rs::Jwe = serde_json::from_str(didcomm_message)?;
    let from_did = jwe.from().as_ref().unwrap();
    let from_key = did_key::resolve(&from_did).unwrap(); // @unwrap!!
    let from_key = did_key::Ed25519KeyPair::from_public_key(&from_key.public_key_bytes());
    let from_key = from_key.get_x25519();

    // 3. Generate elliptic-curve-diffie-hellmann (ecdh) shared secret
    let shared_secret = to_key.key_exchange(&from_key);

    // 4. Decrypt message
    let received = didcomm_rs::Message::receive(didcomm_message, Some(&shared_secret), None);
    let received = received.unwrap(); // @unwrap!
    let received = String::from_utf8(received.body).unwrap(); // @unwrap!

    Ok(format!("{}", received))
}

fn help() -> Result<String, std::io::Error> {
    Ok(String::from("
    Usage: 

        didchat   <init|doc|did|add|send|receive>

        didchat add     <name>    <did>
        didchat send    <to name> <plaintext>
        didchat receive <didcomm encrypted message>
"))
}


#[derive(Debug)]
enum CMD {
    Init,
    Doc,
    Did,
    Add{ name: String, did: String },
    Send{ to: String, message: String },
    Receive{ didcomm_message: String },
    Help
}

pub struct Config {
    cmd: CMD,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let valid_cmds = vec!["help", "did", "doc", "init", "add", "send", "receive"];
        let default_cmd = String::from("help");
        let cmd = args.get(1).unwrap_or(&default_cmd);

        let cmd = if valid_cmds.contains(&&cmd[..]) {
            cmd.clone()
        } else  {
            default_cmd.clone()
        };

        let cmd: CMD = match &cmd[..] {
            "did" => CMD::Did,
            "doc" => CMD::Doc,
            "init" => CMD::Init,
            "add" => {
                let name = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let did = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Add{ name, did }
            },
            "send" => {
                let to = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let message = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Send{ to, message }
            },
            "receive" => {
                let didcomm_message = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Receive{ didcomm_message }
            },
            "help" => CMD::Help,
            &_ => CMD::Help,
        };

        Ok(Config { cmd })
    }
}
