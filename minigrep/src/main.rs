use std::env;
use minigrep::Config;
use std::process;

// general idea: want to create a command line program that runs grep with the following syntax:
// cargo run <searchstring> <filepath> 
// that searches the file located at <filepath> for the occurrence of <searchstring> 

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1)
    }
}
