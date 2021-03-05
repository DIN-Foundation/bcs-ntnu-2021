#[derive(Debug)]
enum CMD {
    Login{ username: String },
    Connect,
    Send,
    Read,
    Help
}



pub struct Config {
    cmd: CMD, // login | connect | send | read | help
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, std::io::Error> {
        let valid_cmds = vec!["help", "login", "connect", "send", "read"];
        let default_cmd = String::from("help");
        let cmd = args.get(1).unwrap_or(&default_cmd);

        let cmd = if valid_cmds.contains(&&cmd[..]) {
            cmd.clone()
        } else  {
            default_cmd.clone()
        };


        let cmd: CMD = match &cmd[..] {
            "login" => {
                let default_name = String::from("anon");
                let name = args.get(2).unwrap_or(&default_name);
                CMD::Login{ username: name.clone()}
            },
            "connect" => CMD::Connect,
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
        CMD::Login{ username } => login(&username),
        CMD::Connect => connect(),
        CMD::Send => send(),
        CMD::Read => read(),
        CMD::Help => help()
    }
}
use std::io::Write;


/**
 * https://tools.ietf.org/html/draft-ietf-jose-cfrg-curves-06#section-2
 */
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Ed25519JWK {
    kty: String, // Must be "OKP"
    crv: String, // Must be "Ed25519"
    x: String,   // base64 encoded public key
    d: String,   // base64 encoded private key
}

impl Ed25519JWK {
    fn new() -> Ed25519JWK {
        let key_private = x25519_dalek::StaticSecret::new(rand_core::OsRng);
        let key_public  = x25519_dalek::PublicKey::from(&key_private);

        let bytes = key_private.to_bytes();
        let x = base64::encode(bytes);

        let bytes = key_public.to_bytes();
        let d = base64::encode(bytes);
        
        Ed25519JWK { kty: String::from("OKP"), crv: String::from("Ed25519"), x, d }
    }

    fn as_json_string(&self) -> Result<String, serde_json::Error> {
        Ok(serde_json::to_string(self)?)
    }
}
/**
 * Login - Creates a public/private key-pair if does not already exists, linked to the given name.
 */ 
fn login(username: &String) -> Result<String, std::io::Error> {
    if !std::fs::metadata(".didchat/user/").is_ok() {
        std::fs::create_dir_all(".didchat/user/").unwrap_or_default();
    }

    let jwk_path = format!(".didchat/user/{}.jwk", username);

    if !std::fs::metadata(&jwk_path).is_ok() {
        let jwk = Ed25519JWK::new();
        let jwk_string = jwk.as_json_string().unwrap();
        let mut file = std::fs::File::create(jwk_path).unwrap();
        file.write(jwk_string.as_bytes()).unwrap();
    }

    Ok(format!("Login {} successful", username))
}

fn connect() -> Result<String, std::io::Error> {
    Ok(String::from("Connecting to friend"))
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

        cargo run <login|connect|send|read|help>
        didchat   <login|connect|send|read|help>

        didchat login <username>
"))
}
