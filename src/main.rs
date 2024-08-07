use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config: Config = Config::new(&args);
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    let contents: String = fs::read_to_string(config.file_path)
        .expect("Unable to read file");
    println!("Got contents: {contents}")
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        Config { query, file_path }
    }
}
