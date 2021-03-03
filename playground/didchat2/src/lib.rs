// @see https://stackoverflow.com/a/39070533
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
custom_derive! {
    #[derive(Debug, EnumFromStr)]
    enum CMD {
        Login,
        Connect,
        Send,
        Read,
        Help
    }
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

        // Uppercase first letter before mapping to enum.
        let cmd = uppercase_first_letter(&cmd);
        let cmd: CMD = cmd.parse().unwrap();

        Ok(Config { cmd })
    }
}

pub fn run(config: Config) -> Result<String, std::io::Error> {
    match config.cmd {
        CMD::Login => login(),
        CMD::Connect => connect(),
        CMD::Send => send(),
        CMD::Read => read(),
        CMD::Help => help()
    }
}

fn login() -> Result<String, std::io::Error> {
    Ok(String::from("Login to user"))
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
"))
}

// @see https://stackoverflow.com/a/38406885
fn uppercase_first_letter(s: &str) -> String {
    let lowercase = s.to_lowercase();
    let mut c = lowercase.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
