use std::io;
use std::fs::File;
use std::io::prelude::*;

pub fn create_text() -> io::Result<()> {
    let mut file = File::create("test.txt")?;
    file.write_all(b"This is a test in creating and writing a file in Rust")?;
    return Ok(())
}

pub fn read_text() -> io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "This is a test in creating and writing a file in Rust");

    println!("{}", contents);
    return Ok(())
}