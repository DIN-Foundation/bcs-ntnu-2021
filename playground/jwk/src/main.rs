fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = jwk::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Config::new(&args) failed: {}", err);
        std::process::exit(1);
    });

    let output = jwk::run(config).unwrap_or_else(|err| {
        eprintln!("run(config) failed: {}", err);
        std::process::exit(2);
    });

    println!("{}", output);
}
