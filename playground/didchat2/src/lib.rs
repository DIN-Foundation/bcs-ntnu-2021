#[derive(Debug)]
enum CMD {
    Doc,
    Did,
    Login,
    Friend,
    Send,
    Read,
    Help
}

pub struct Config {
    cmd: CMD,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let valid_cmds = vec!["help", "did", "doc", "login", "friend", "send", "read"];
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
            "login" => CMD::Login,
            "friend" => CMD::Friend,
            "send" => CMD::Send,
            "read" => CMD::Read,
            "help" => CMD::Help,
            &_ => CMD::Help,
        };

        Ok(Config { cmd })
    }
}

pub fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        CMD::Login => login(),
        CMD::Doc => doc(),
        CMD::Did => did(),
        CMD::Friend => friend(),
        CMD::Send => send(),
        CMD::Read => read(),
        CMD::Help => help()
    }
}

/**
 * Login - Creates a public/private key-pair if does not already exists, linked to the given name.
 */ 
fn login() -> Result<String, std::io::Error> {
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


fn friend() -> Result<String, std::io::Error> {
    Ok(String::from("Added friend"))
}

fn send() -> Result<String, std::io::Error> {
    Ok(String::from("Sending message"))
}

fn read() -> Result<String, std::io::Error> {
    Ok(String::from("Reading message"))
}

fn help() -> Result<String, std::io::Error> {
    Ok(String::from("
    Usage: 

        cargo run <login|doc|friend|send|read|help>
        didchat   <login|doc|friend|send|read|help>

        didchat login <username>
"))
}
