use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query: &str = &args[1];
    let file_path: &str = &args[2];

    println!("Searching for {query}");
    println!("in file {file_path}");
}
