use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let my_name = String::from("Illyasen");
    println!("Hello, {}!", my_name);
    let some_string = something(String::from("This is some string idk"));
    println!("{}", some_string);

    let _created_file = create_text();
    let _read_file = read_text();
}

fn something(my_string: String) -> String {
    String::from(my_string)
}

fn create_text() -> io::Result<()> {
    let mut file = File::create("test.txt")?;
    file.write_all(b"This is a test in creating and writing a file in Rust")?;
    return Ok(())
}

fn read_text() -> io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    assert_eq!(contents, "This is a test in creating and writing a file in Rust");

    println!("{}", contents);
    return Ok(())
}

