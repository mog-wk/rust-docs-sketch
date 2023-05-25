use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("less than 3 args")
        }
        Ok(Self { query: args[2].clone(), filename: args[1].clone() })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.filename)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    // &str.lines returns an iter() based on \n
    for line in content.lines() {
        if line.contains(query) {
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
        let c = Config::new(&data).unwrap_or_else(|err| {
            panic!("");
        });
    }
}
