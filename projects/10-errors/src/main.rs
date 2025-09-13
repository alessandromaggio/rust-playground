// You can use this syntax in Cargo.toml to avoid unwinding
// This will make the binary smaller and leave up to the OS to clean up the stack
//
// [profile.release]
// panic = 'abort'
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file_result = File::open("hello.txt");

    let mut username_file = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e), // propagate the error
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e), // propagate the error
    }
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // the ? operator propagates the error if any

    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // the ? operator propagates the error if any
    Ok(username)
}

fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?; // the ? operator propagates the error if any
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt") // the ? operator is implicit in the return type
}

fn main() {
    let file_result = File::open("hello.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let greeting_file = File::open("hello.txt").unwrap(); // will panic if there's an error
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project"); // will panic with custom message if there's an error

    // unrecoverable error
    panic!("crash and burn");
}
