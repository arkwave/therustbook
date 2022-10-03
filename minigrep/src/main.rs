use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {} in {}", config.query, config.filepath);

    run(config);
}

fn run(config: Config) {
    let contents =
        fs::read_to_string(config.filepath).expect("Something went wrong when reading the file!");
    println!("Found contents:\n{}", contents)
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
