use std::error::Error; 
use std::fs;
use std::env; 

// Represent the valid states of the program as a struct; in this case, we need a query and a filepath. 
// Important: setting the struct to pub doesn't turn the individual fields public as well. This needs
// to be done individually. 
pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool 
}

// create an impl containing methods to 1) create a valid Config defining a run and 2) validate arguments. 
impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok(); 
        println!("Ignoring case: {}", ignore_case);

        Ok(Config { query, filepath, ignore_case })
    }
}

// define an overall run method that pulls the invidual logic pieces together. The return type of the error,
// Box <dyn Error>, indicates to the compiler that the return type implements the Error trait without
// having to specify what the exact error type is. 
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

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
    return results 
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase(); // this turns query back into a String
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { // requires a borrow, i.e. &query instead of query
            results.push(line)
        }
    }
    return results 
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

    #[test] 
    fn case_insensitive() {
        let query = "rUST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }

}


