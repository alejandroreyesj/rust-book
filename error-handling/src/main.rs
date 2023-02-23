use std::fs::File;
use std::io::{self, ErrorKind, Read};
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let greeting_file_result = File::open("greeting.txt");
    let greeting_files = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("greeting.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    read_username_from_file().unwrap();
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");
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
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
fn read_username_from_file_4() -> Result<String, io::Error> {
     fs::read_to_string("username.txt")
}

