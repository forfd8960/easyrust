use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1].clone();
        let file_path = &args[2].clone();

        Ok(Config { query: query.to_string(), file_path: file_path.to_string() })
    }
}

fn run(config: Config) {
    let content = fs::read_to_string(config.file_path).expect("unable to read the fiel");
    println!("read content\n {content}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("parsing arguments: {err}");
        process::exit(1);
    });

    println!("query: {}, file_path: {}", config.query, config.file_path);
    run(config);
}    
