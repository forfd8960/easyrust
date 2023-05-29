use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn new(args: &[String]) -> Self {
        if args.len() < 3 {
            panic("not enough arguments");
        }

        let query = &args[1].clone();
        let file_path = &args[2].clone();

        Config { query, file_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("query: {}, file_path: {}", config.query, config.file_path);
    let content = fs::read_to_string(config.file_path).expect("unable to read the fiel");
    println!("read content\n {content}");
}
