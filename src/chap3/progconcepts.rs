pub fn concepts() {
    println!("Rust Book: Chapter 3");

    let _this_be_immutable = 20;
    // _this_be_immutable = 21; // error: cannot assign twice
    let mut _this_be_mutable = 20;
    _this_be_mutable = 21; // this works because this is mutable

    const _PIE: f64 = 3.14;

    let x = 5;
    println!("Value of x: {}", x); // result: 5
    let x = x + 5;
    println!("Value of x: {}", x); // result: 10

    // TODO: Data Types - https://doc.rust-lang.org/book/ch03-02-data-types.html
}

/*
    Usually variables in Rust are by default immutable unless you want to change it to be mutable by adding the keyword "mut" before the variable name

    Variable - storage for a value ex. strings and numbers

    Immutability - you cant change the value
    
    Mutability - you can modify the value
    
    Constants - unchangable values can cannot use "mut", in const you must add the type to the variable, and constants are named in UPPER_SNAKE_CASE
    
    Shadowing - declaring a variable of the same name and taking the value of the previous one as the last variable is being overshadowed by the new one

    Difference between Shadowing and Mutability is in shadowing you can use the value of the previous variable but with being mutable you cant use the value of the previous variable


*/