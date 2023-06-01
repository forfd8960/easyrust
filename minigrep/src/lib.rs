use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1].clone();
        let file_path = &args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case: ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    println!("read content\n {content}");
    
    let results = if config.ignore_case {
        search_case_insenstitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    
    for line in results {
        println!("search result: {line}");
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line.trim());
        }
    }
   res 
}

pub fn search_case_insenstitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line.trim());
        }
    }
   res 
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUST";
        let contents = "\
        Rust:
        safe, fast, productive.
        Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insenstitive(query, contents));
    }
}
