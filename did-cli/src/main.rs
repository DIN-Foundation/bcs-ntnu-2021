#[async_std::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = did::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Config::new(&args) failed: {}", err);
        std::process::exit(1);
    });

    let output = did::run(config).await.unwrap_or_else(|err| {
        eprintln!("run(config) failed: {}", err);
        std::process::exit(2);
    });

    println!("{}", output);
}
