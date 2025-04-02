use chrono;
use rand::Rng;
use std::{time, thread};

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

    // Integers have 2 types: signed(w/ negative) and unsigned(w/o negative)
    let _some_int: i32 = -32; 
    let _some_unsigned_int: u32 = 32;
    let _some_float: f64 = -35.5; // floats can be positive and/or negative

    // Basic Arithmetic
    let _add_num = 1 + 1;
    let _sub_num = 2 - 1;
    let _mul_num = 5 * 5;
    let _div_num = 10 / 2;
    let _remain_num = 10 % 3;

    // Booleans - values that are true or false, booleans are usefull in conditionals like if..else
    let _some_true = true;
    let _some_false = false;

    // Characters - single alphabetic characters, unicode, and characters from different languages
    let _some_letter = 'R';
    let _some_smiley_face = '😀';

    // Tuples - grouping values with different types, tuples cannot grow in size
    let _some_tuple: (i32, char, bool) = (32, 'C', false);

    // values in a tuple can be accessed through index
    println!("Tuple Values: {}, {}, {}", _some_tuple.0, _some_tuple.1, _some_tuple.2);

    let _pacific_ocean_coords = (8.7832, 124.5085);
    let (_pacific_south, _pacific_north) = _pacific_ocean_coords; // the coords in the tuple are binded to _south and _north, and this is a form of destructuring
    println!("Pacific Coordinates: South - {}°, North - {}°", _pacific_south, _pacific_north);

    let _empty_tuple = (); // a tuple w/o values has a special name, "unit" with a type of "()"

    // Arrays - collection of values, values must have the same type, arrays in Rust have a fixed length, values in a array are stored on the Heap rather than the Stack
    let _some_array = [5, 10, 15, 20, 25];

    // Arrays are used if you know that the values inside it wont change
    let _days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

    // accessing array elements
    println!("This day is {}", _days[3]);

    for day in _days {
        println!("Day: {}", day);
    }

    // you can also be explicit with the type and length of an array
    let _array_of_five: [i32; 5] = [1, 2, 3, 4, 5];

    // accessing a non-existent element
    // println!("{}", _days[10]) // this will panic since the index is out of bounds

    // calling the function
    time_fn();

    // values that are passed in a function is called a "parameter"
    println!("Res: {}", add_num(5, 20));

    // if..else loop
    let _some_val = rand::thread_rng().gen_range(1..=200);
    if _some_val < 100 {
        println!("{} is less than 100", _some_val);
    }
    else if _some_val > 100 {
        println!("{} is greater than 100", _some_val);
    }
    else {
        println!("Value is not a number");
    }

    // if..let statement - since if is an expression we can use it inside of a let variable
    let _conditon = true;
    let _number = if _conditon { 5 } else { 6 }; // the number is bound to this variable based on the condition

    println!("Number is {}", _number);

    // loop - executes forever or until you explicitly tell it to stop
    let mut i = 3;
    loop {
        let seconds = time::Duration::from_secs(1);
        println!("Countdown: {}", i);
        i -= 1;
        thread::sleep(seconds);

        // here I explicitly added a break to break out of the loop
        if i == 0 {
            break println!("Done looping");
        }
    }

    // loop with a returning value
    let mut _counter = 0;
    let _return_from_loop = loop {
        _counter += 1;
        
        if _counter == 5 {
            break _counter * 2
        }
    }; // returns a value of 10

    println!("Returned value from loop: {}", _return_from_loop);

    // TODO: Loops with Labels - https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops
}

// creating a function
fn time_fn() {
    // this is a statement
    let chrono_time_now = chrono::Local::now().format("%H:%M:%S");
    
    // this is an expression
    println!("Chrono time now: {}", chrono_time_now);
}

// a function with arguments and a return value, you can also remove the "return" keyword as long as there is an explicit return type
// when adding parameters you must declare the type
fn add_num(a: i32, b: i32) -> i32 {
    a + b
}

/*
    Usually variables in Rust are by default immutable unless you want to change it to be mutable by adding the keyword "mut" before the variable name

    Variable - storage for a value ex. strings and numbers

    Immutability - you cant change the value
    
    Mutability - you can modify the value
    
    Constants - unchangable values can cannot use "mut", in const you must add the type to the variable, and constants are named in UPPER_SNAKE_CASE
    
    Shadowing - declaring a variable of the same name and taking the value of the previous one as the last variable is being overshadowed by the new one

    Difference between Shadowing and Mutability is in shadowing you can use the value of the previous variable but with being mutable you cant use the value of the previous variable

    Scalar Types - represents a single value, and there are 4 types Integer, Floating point number, Booleans, and Characters

    Integers & Floats can occupy 8, 16, 32, 64, and 128 bits of space while usize and isize depends on the architecture of the system

    Compound Types - can group multiple values into one type, Rust has 2 compound types: Tuples and Arrays

    Heap Memory - is a dynamic option for storing memory that can be resized and continue to retain the value even after the scope ends

    Stack Memory - is automatically allocated and deallocated and essentially stores the local variables within the function's scope

    Function - a block of code that will run if called, you can also add parameters inside "()" and also have return types ex. fn some_fn(a: i32) -> i32 {...}

    Statements - are instructions that perform some action and do not return a value.
    
    Expressions - evaluate to a resultant value. Let’s look at some examples.

    Comments - mostly used for documentation and explenation of complex code, comments wont be seen by the compiler. Comments are written after double slashes // or between /**/

    Control Flow - run code depdending whether a condition is true, there is if..else loop, for loop, loop, and while loop

    If you have multiple if..else blocks its best to refactor your if..else code with match statement
*/