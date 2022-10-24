use std::error::Error; 
use std::{fs, env};
// use std::env; 

// Represent the valid states of the program as a struct; in this case, we need a query, filepath and ignore_case. 
// Important: setting the struct to pub doesn't turn the individual fields public as well. This needs
// to be done individually. 
pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool 
}

// create an impl containing methods to 1) create a valid Config defining a run and 2) validate arguments. 
impl Config {
    // function signature specifies the following:
    // 1) args is mutable - required, since we need it to implement the Iterator type, and the
    //    Iterator type implements a next method that mutates the iterator. 
    // 2) The item type contained within the iterator is String 
    // 3) function returns a Result enum which either contains a Config struct, or a reference to a string 
    //    with the lifetime of the entire program (indicated by the 'static lifetime annotation);
    //    this string is basically the error message. 
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config,&'static str> {
        args.next(); // want to skip filename
         
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string found!")    
        };
        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("No filepath found!")
        };

      // let ignore_case = match args.next() {
      //      Some(arg) => {
      //          if arg == "-c" {
      //              true 
      //          } else {
      //              false 
      //          }
      //      },
      //      None => false
      //  };
        eprintln!("Ignoring case: {}", ignore_case);

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
// for the same reason, we don't need a lifetime annotation on query. 
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
   contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

   // old implementation without iterators and closures 
   // let mut results = Vec::new(); 
   // for line in contents.lines() {
   //     if line.contains(query) {
   //         results.push(line)
   //     }
   // }
   // return results 
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
   let query = query.to_lowercase();
   contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()

   // old interpretation with no iterators
   // let mut results = Vec::new();
   // let query = query.to_lowercase();
   // for line in contents.lines() {
   //     if line.to_lowercase().contains(&query) {
   //         results.push(line)
   //     }
   // }
   // return results
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


