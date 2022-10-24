use std::env;
use minigrep::Config;
use std::process;

// general idea: want to create a command line program that runs grep with the following syntax:
// cargo run <searchstring> <filepath> 
// that searches the file located at <filepath> for the occurrence of <searchstring> 

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err); // redirects error messages to stderr rather than stdout
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    }
}
