//
// Entry point of the application
//
fn main() {
    // Although we very rarely need to annotate types in Rust, collect is one 
    // function you do often need to annotate because Rust isn’t able to infer 
    // the kind of collection you want.
    let args: Vec<String> = std::env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Config::new(&args) failed: {}", err);
        std::process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("run(config) failed: {}", err);
        std::process::exit(2);
    }
}
