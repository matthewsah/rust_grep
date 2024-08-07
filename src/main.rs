use std::env;
use std::process;
use rust_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // what is unwrap or else ||
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // error handling - we don't need the result returned from Ok
    if let Err(e) = rust_grep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
