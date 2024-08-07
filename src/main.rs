use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query: &str = &args[1];
    let file_path: &str = &args[2];

    println!("Searching for {query}");
    println!("in file {file_path}");

    let contents: String = fs::read_to_string(file_path).expect("Unable to read file");
    println!("Got contents: {contents}")
}
