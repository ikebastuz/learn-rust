use core::panic;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    // println!("Hello, world!");
    // panic!("Crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    let file_name = "hello.txt";
    let greeting_file_result = File::open(&file_name);

    // Variant 1
    // let _greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create(&file_name) {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening file: {:?}", other_error);
    //         }
    //     },
    // };

    // Variant 2
    // let _greeting_file = File::open(file_name).unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create(file_name).unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    match read_username_from_file() {
        Ok(name) => println!("Name: {}", name),
        Err(err) => panic!("Error: {:?}", err),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // Long way
    // let username_file_result = File::open("hello.txt");
    //
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    //
    // let mut username = String::new();
    //
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // Short way
    // let mut username_file = File::open("hellio.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // The shortest way
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)

    // Ultimate
    // fs::read_to_string("hello.txt")
}
