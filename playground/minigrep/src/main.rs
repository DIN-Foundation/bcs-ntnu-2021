//
// Entry point of the application
//
fn main() {
    // Although we very rarely need to annotate types in Rust, collect is one 
    // function you do often need to annotate because Rust isn’t able to infer 
    // the kind of collection you want.
    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Config::new failed: {}", err);
        std::process::exit(1);
    });

    run(config);
}

fn run(config: Config) {
    // Read file
    let file = std::fs::read_to_string(&config.filename)
        .expect(&format!("Failed to read: {}", config.filename));

    println!(
        "{{ \"query\": \"{}\", \"config.filename\": \"{}\" }}", 
        config.query, 
        config.filename
    );

    println!("\n{}", file);    
}

struct Config {
    _program_name: String,
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("Wrong number of args. Usage: cargo run <query> <filename>");
        }
        let program_name = args[0].clone();
        let query        = args[1].clone();
        let filename     = args[2].clone();

        Ok(Config { _program_name: program_name, query, filename })
    }
}
