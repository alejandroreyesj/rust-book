use std::error::Error;
use std::{env, fs};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Self {
            query: &args[1],
            file_path: &args[2],
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    };
    results.into_iter().for_each(|line| {
        println!("{line}");
    });
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.contains(query))
        .collect::<Vec<&str>>()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "DuCt";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, contents)
        );
    }
}
