use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("args: {:?}", args);
    println!("query: {}, file_path: {}", query, file_path);
    let content = fs::read_to_string(file_path).expect("unable to read the fiel");
    println!("read content\n: {content}");
}
