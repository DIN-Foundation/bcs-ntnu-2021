//
// Instrument with 'EnumFromStr' trait, to be able to parse enum from string.
//
// @see https://stackoverflow.com/a/39070533
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
custom_derive! {
    // format!("{}") uses 'Display', format!("{:?}") uses 'Debug'
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


pub fn run(config: Config) -> Result<(), std::io::Error> {
    match config.cmd {
        CMD::Login => login(),
        CMD::Connect => connect(),
        CMD::Send => send(),
        CMD::Read => read(),
        CMD::Help => help()
    }
}

fn login() -> Result<(), std::io::Error> {
    println!("Login to user");
    Ok(())
}

fn connect() -> Result<(), std::io::Error> {
    println!("Connecting to friend");
    Ok(())
}

fn send() -> Result<(), std::io::Error> {
    println!("Sending message");
    Ok(())
}

fn read() -> Result<(), std::io::Error> {
    println!("Reading message");
    Ok(())
}

fn help() -> Result<(), std::io::Error> {
    println!("
    Usage: 

        cargo run <login|connect|send|read|help>
        didchat   <login|connect|send|read|help>
");
    Ok(())
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
