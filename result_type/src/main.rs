use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() -> Result<(), Box<dyn Error>> {
    match read_username_from_file() {
        Ok(username) => println!("Username: {username}"),
        Err(e) => panic!("Error: {e}")
    } 
    match read_username_from_file_shorter() {
        Ok(username) => println!("Username: {username}"),
        Err(e) => panic!("Error: {e}")
    }
    match read_username_from_file_shortest() {
        Ok(username) => println!("Username: {username}"),
        Err(e) => panic!("Error: {e}")
    }
    match last_char_of_first_line("test") {
        Some(ch) => println!("Char: {ch}"),
        None => panic!("No char")
    }
    File::open("hello.txt")?;
    Ok(())
}
