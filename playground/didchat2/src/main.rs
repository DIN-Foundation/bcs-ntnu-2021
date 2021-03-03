fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = didchat2::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Config::new(&args) failed: {}", err);
        std::process::exit(1);
    });

    if let Err(err) = didchat2::run(config) {
        eprintln!("run(config) failed: {}", err);
        std::process::exit(2);
    }
}
