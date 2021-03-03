
pub struct Config {
    cmd: String, // login | connect | send | receive | help
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn std::error::Error>> {

        let default_cmd = String::from("help");
        let cmd = args.get(1).unwrap_or(&default_cmd);

        Ok(Config { cmd: cmd.clone() })
    }
}