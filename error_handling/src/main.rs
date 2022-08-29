use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    println!("Hello, world!");
}

fn read_username_from_file_basic() -> Result<String, io::Error> {
    // Example of very explicit error handling. Process is as follows:
    // 1) Try to read hello.txt with File::open("hello.txt"); this returns a Result enum.
    // 2) Handle both cases of the Result enum explicitly, returning the error if it occurs.
    // 3) Create a "username" string, and try to load the contents of the file into the username
    //    variable.
    // 4) file.read_to_string accepts a mutable reference to a string, and returns a Return enum;
    //    need to explicitly handle the Ok and Err.
    //
    // Key fact here is that the error is RETURNED if obtained - the errors are propagated up to
    // the code that calls this function.
    //
    let username_file_result = File::open("hello.txt");

    // try to read the username file, explicitly handle both cases.
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // create username string, and try to read the contents of username file to
    // the string, handling errors appropriately.
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_better() -> Result<String, io::Error> {
    // This implementation does the same thing, except it uses the ? operator. Under the hood,
    // errors that have the ? operator called on them go through the `from` function defined in the
    // From trait in the standard library, which converts values from one type to another. When the
    // ? operator is called on the Err variant, it converts the Err variant into the error type
    // specified in the return type of the function.
    //
    // Could defile our own error named OurError and have it impl From<io::Error> for OurError to
    // construct an instance of OurError from io::Error with the ? operator.
    //
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_chained() -> Result<String, io::Error> {
    // This variant does the same as read_username_from_file_better but instead of having one
    // function call per line, it just chains the calls.

    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_best() -> Result<String, io::Error> {
    // this is the best implementation because it leverages the in-built fs::read_to_string method,
    // that takes care of all the error handling for us.
    fs::read_to_string("hello.txt")
}

//fn read_username_failed_impl() -> Result<String, io::Error> {
//  let greeting_file = File::open("hello.txt")?;
//}
