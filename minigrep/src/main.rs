use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("parsing arguments: {err}");
        process::exit(1);
    });

    println!("query: {}, file_path: {}", config.query, config.file_path);
    match run(config) {
        Ok(()) => println!("sucess read content"),
        Err(e) => println!(" has error: {e} when reading file"),
    };
}
