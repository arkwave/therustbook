use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // returns a Result object. references are immutable by default, so &mut returns a mutable reference. 
            .expect("Failed to read line"); // .expect() crashes if Result is of variant Err, returns value if Ok. 
        
        // note: read_line() call saves the result of read_line to the buffer specified by a mutable reference to <guess>.  mutable refernce
        // guarantees that this function has exclusive access to the guess variable. 
    
        // shadowing: we are "borrowing" the name "guess" from the previously defined str value and replacing it with a u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue, // _ is a catch-all value. 
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("Ayylmao");
                break;
            }
        }

    }
    
}
