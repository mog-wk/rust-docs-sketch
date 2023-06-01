use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;


#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
        args.next();
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a file name")
        };

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a query string"),
        };


        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok( Self { query, filename, case_sensitive, } )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let result = if config.case_sensitive { 
        search(&config.query, &content)
    } else {
        search_insensitive(&config.query, &content)
    };

    for line in result.iter() {
        println!("{}", &line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
        .filter(|line| line.contains(query))
        .collect()
}

// run CASE_INSENSITIVE=1 cargo run ..
pub fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duck";
        let content = "\
        Rust:
        safe ,fast, duck
        ";
        assert_eq!(vec!["safe, fast, duck"], search(query, content));
    }

    #[test]
    #[should_panic]
    fn parse_invalid_file() {
        let data = vec!["pog".to_string(), "pepega".to_string()];
        let _c = Config::new(&data).unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }

    #[test]
    fn case_sensitive() {
        let query = "duck";
        let content = "\
        Rust:
        SAFE ,FAST, DUCK
        ";
        assert_eq!(vec!["", ""], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "duck";
        let content = "\
        Rust:
        SAFE ,FAST, duck
        ";
        assert_eq!(vec!["SAFE, FAST, DUCK"], search_insensitive(query, content));
    }
}
