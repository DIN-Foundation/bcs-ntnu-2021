pub fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        CMD::New => new(),
        CMD::Doc => doc(),
        CMD::Did => did(),
        CMD::Add{ alias, did } => add(&alias, &did),
        CMD::Send{ alias, message } => send(&alias, &message),
        CMD::Read{ alias, message } => read(&alias, &message),
        CMD::Help => help()
    }
}

/**
 * new - Creates a public/private key-pair if does not already exists, effectively creating a new chat.
 */ 
fn new() -> Result<String, std::io::Error> {
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
        
        Ok(format!("New user signed in."))
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

fn send(alias: &str, message: &str) -> Result<String, std::io::Error> {
    use did_key::Ecdh;
    use did_key::KeyMaterial;
    use did_key::DIDCore;

    // 1. Read Ed25519 keys from file
    let my_seed = std::fs::read(".didchat/seed").unwrap(); // @unwrap!!
    let my_key = did_key::Ed25519KeyPair::from_seed(&my_seed);
    let my_did = my_key.get_did_document(did_key::CONFIG_LD_PUBLIC).id;
    let other_did = std::fs::read_to_string(format!(".didchat/dids/{}.did", alias)).unwrap(); // @unwrap!!
    let other_key = did_key::resolve(&other_did).unwrap(); // @unwrap!!
    let other_key = did_key::Ed25519KeyPair::from_public_key(&other_key.public_key_bytes());

    // 2. Transform keys from Ed25519 -> x25519
    let my_key = my_key.get_x25519();
    let other_key = other_key.get_x25519();

    // 3. Generate elliptic-curve-diffie-hellmann (ecdh) shared secret
    let shared_secret = my_key.key_exchange(&other_key);
    println!("{:?}", shared_secret);

    // 4. Construct didcomm message
    let message = didcomm_rs::Message::new()
        .from(&my_did)
        .to(vec![&other_did])
        .timed(Some(3600))
        .body(message.as_bytes())
        .as_jwe(&didcomm_rs::crypto::CryptoAlgorithm::XC20P);

    // 5. Seal/encrypt message using shared secret
    let sealed_message = message
        .seal(&shared_secret)
        .unwrap(); // @unwrap!!

    Ok(format!("{}", &sealed_message))
}

fn read(_alias: &str, _message: &str) -> Result<String, std::io::Error> {

    Ok(String::from("Reading message"))
}

fn help() -> Result<String, std::io::Error> {
    Ok(String::from("
    Usage: 

        didchat   <new|doc|did|add|send|read|help>

        didchat add <alias> <did:key:etc....>
        didchat send <alias> <message>
        didchat read <alias> <message>
"))
}


#[derive(Debug)]
enum CMD {
    New,
    Doc,
    Did,
    Add{ alias: String, did: String },
    Send{ alias: String, message: String },
    Read{ alias: String, message: String },
    Help
}

pub struct Config {
    cmd: CMD,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let valid_cmds = vec!["help", "did", "doc", "new", "add", "send", "read"];
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
            "new" => CMD::New,
            "add" => {
                let alias = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let did = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Add{ alias, did }
            },
            "send" => {
                let alias = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let message = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Send{ alias, message }
            },
            "read" => {
                let alias = (match args.get(2) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                let message = (match args.get(3) {
                    Some(arg) => arg,
                    None => return Ok(Config{ cmd: CMD::Help }),
                }).clone();

                CMD::Read{ alias, message }
            },
            "help" => CMD::Help,
            &_ => CMD::Help,
        };

        Ok(Config { cmd })
    }
}
