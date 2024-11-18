use std::error::Error;
// collect
use std::cell::RefCell;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename.take())?;
    println!("with text :\n{}", contents);
    // search
    println!("results:{:#?}", search(&config.query.borrow(), &contents));
    Ok(())
}

pub trait FileConfiguration {
    fn push_variable(&self, args: Vec<String>) -> Result<(), &'static str>;
}

impl FileConfiguration for Config {
    fn push_variable(&self, args: Vec<String>) -> Result<(), &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        self.query.borrow_mut().push_str(&args[1]);
        self.filename.borrow_mut().push_str(&args[2]);
        Ok(())
    }
}

pub struct Config {
    pub query: RefCell<String>,
    pub filename: RefCell<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            query: RefCell::new(String::new()),
            filename: RefCell::new(String::new()),
        }
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
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
        let query = "lbk";
        let contents = "\
Rust:
safe,fast,call,lbk
lak";
        assert_eq!(vec!["safe,fast,call,lbk"], search(query, contents))
    }
}
