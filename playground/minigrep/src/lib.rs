
pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("Wrong number of args. Usage: cargo run <query> <filename>");
        }
        let query        = args[1].clone();
        let filename     = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let _file = std::fs::read_to_string(&config.filename)?;

    Ok(())    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Da doo";

        let file = "\
Gitchy, gitchy
Ooh la la
Da doo ron ron
You won't get far
I'm machine
I'm obsolete
In the land of the free
Lobotomy
I wanna suck, I wanna lick
I want to cry and I want to spit
Tears of pleasure
Tears of pain
They trickle down your face the same";

        assert_eq!(vec!["Da doo ron ron"], search(query, file));
    }
} 