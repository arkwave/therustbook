use std::error::Error; 
use std::fs;

// Represent the valid states of the program as a struct; in this case, we need a query and a filepath. 
// Important: setting the struct to pub doesn't turn the individual fields public as well. This needs
// to be done individually. 
pub struct Config {
    pub query: String,
    pub filepath: String,
}

// create an impl containing methods to 1) create a valid Config defining a run and 2) validate arguments. 
impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { query, filepath })
    }
}

// define an overall run method that pulls the invidual logic pieces together. The return type of the error,
// Box <dyn Error>, indicates to the compiler that the return type implements the Error trait without
// having to specify what the exact error type is. 
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents =
        fs::read_to_string(config.filepath)?;
    Ok(())
}

// in this case, we need the explicit lifetime annotation on contents because
// the lifetime of the return value depends on the lifetime of the reference in contents. 
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); 
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    return vec![]
}

// adding in some tests
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
}


