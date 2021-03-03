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
        Receive,
        Help
    }
}


pub struct Config {
    cmd: CMD, // login | connect | send | receive | help
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn std::error::Error>> {

        //
        // Parse <cmd>
        //
        let valid_cmds = vec!["help", "login", "connect", "send", "receive"];
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
        CMD::Login => println!("Login to user"),
        CMD::Connect => println!("Connecting to friend"),
        CMD::Send => println!("Sending message"),
        CMD::Receive => println!("Receiving message"),
        CMD::Help => println!("
Didchat
    Usage: cargo run <login|connect|send|receive|help>
")
    }

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
