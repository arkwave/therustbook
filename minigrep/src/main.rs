use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filepath = &args[2];
    let contents = fs::read_to_string("poem.txt").expect("should be able to read file...");

    println!("Searching for {}", query);
    println!("In {}", filepath);
    println!("With content:\n {}", contents);
}

struct Config {
    query: String,
    filepath: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        let query = args[1].clone();
        let filepath = args[2].clone();

        Ok(Config { query, filepath })
    }
}
