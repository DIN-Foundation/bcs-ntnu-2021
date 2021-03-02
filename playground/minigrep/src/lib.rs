
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("Wrong number of args. Usage: cargo run <filename> <query>");
        }
        let filename  = args[1].clone();
        let query     = args[2].clone();

        let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

//
// Runs the program
//
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search_sensitive(&config.query, &file)
    } else {
        search_insensitive(&config.query, &file)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())    
}

//
// Searches a file, returning only the lines that match the query
//
pub fn search_sensitive<'a>(query: &str, file: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in file.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

//
// Searches a file case-insensitively
//
pub fn search_insensitive<'a>(query: &str, file: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in file.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        } 
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
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

        assert_eq!(vec!["Da doo ron ron"], search_sensitive(query, file));
    }

    #[test]
    fn case_insensitive() {
        let query = "da doo";

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

        assert_eq!(vec!["Da doo ron ron"], search_insensitive(query, file));
    }
} 
