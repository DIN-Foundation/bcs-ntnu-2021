#[derive(Debug)]
enum CMD {
    New,
    Doc,
    Did,
    Add{ name: String, did: String },
    Send,
    Read,
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
        CMD::New => new(),
        CMD::Doc => doc(),
        CMD::Did => did(),
        CMD::Add{ name, did } => add(&name, &did),
        CMD::Send => send(),
        CMD::Read => read(),
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

fn add(name: &str, did: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    if !std::fs::metadata(".didchat/dids/").is_ok() {
        std::fs::create_dir_all(".didchat/dids/").unwrap_or_default();
    }

    let friend_path = format!(".didchat/dids/{}.did", name);
    let mut file = std::fs::File::create(friend_path).unwrap();
    file.write(did.as_bytes()).unwrap();

    Ok(format!(".didchat/dids/{}.did", name))
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

        didchat   <new|doc|did|add|send|read|help>

        didchat new
        didchat add <alias> <did:key:etc....>
"))
}
