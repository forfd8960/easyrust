use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = &args[1];
    let file_path = &args[2];

   Config {query: query.to_string(), file_path: file_path.to_string()}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("query: {}, file_path: {}", config.query, config.file_path);
    let content = fs::read_to_string(config.file_path).expect("unable to read the fiel");
    println!("read content\n {content}");
}
