use std::fs::File;
use std::io::ErrorKind;

use std::io::{self, Read};
use std::fs;

fn main() {
    // let f: u32 = File::open("hello.txt");    // can check with type

    let greeting_file_result = File::open("hello.txt");            // return Result<File, Error>

    // let greeting_file_result = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("There was a problem opening the file: {:?}", error),
    // };

    // Matching on different error
    let greeting_file_result = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {                                          
            ErrorKind::NotFound => match File::create("hello.txt") {                // create File
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),             // error when create File
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);              // other error, except NotFound
            }            
        },
    };

    // simple version with unwrap
    let greeting_file_result = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error)
        }
    });

    // unwrap and expect
    // let greeting_file = File::open("hello3.txt").unwrap();                          // unwrap have same message

    // let greeting_file = File::open("hello3.txt")
    //             .expect("hello3.txt should be included in this project");           // set error message

    let some_error = read_username_from_file();
    println!("The error is {:?}", some_error);

    let user_name = read_username_from_file_simple();
    println!("The username is {:?}", user_name.unwrap());

    let some_error2 = read_username_from_file_even_simple();
    println!("The error is {:?}", some_error2);

    let user_name = read_username_from_file_use_fs();
    println!("The username is {:?}", user_name.unwrap());
    println!();

    let char = last_char_of_first_line("what is the last char
                                        test");
    println!("The char is {:?}", char);

    let char = last_char_of_first_line("
                                        test");
    println!("The char is {:?}", char);

    // let greeting_file = File::open("hello.txt")?;  // it works when main function's output is Result<(), Box<dyn Error>>
    // Ok(())
}

// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello3.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// simple version with ?, use From trait, to one error type
fn read_username_from_file_simple() -> Result<String, io::Error> {
    let mut username_file = File::open("hello2.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// even simple version
fn read_username_from_file_even_simple() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello3.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// use fs::read_to_string
fn read_username_from_file_use_fs() -> Result<String, io::Error> {
    fs::read_to_string("hello2.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}