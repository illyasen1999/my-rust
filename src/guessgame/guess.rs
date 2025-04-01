use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn guessing_game() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=50);
    let mut tries = 0;

    loop {
        println!("Please input your guess 1-50: ");
    
        let mut guess = String::new();

        // user input handling
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // conversion of string to number w/ error handling
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // comparison of input and answer
        match guess.cmp(&secret_number){
            Ordering::Less => {
                tries += 1;
                println!("Too Small | tries: {} \n", tries);
            },
            Ordering::Greater => {
                tries += 1;
                println!("Too Big | tries: {} \n", tries);
            },
            Ordering::Equal => {
                tries += 1;
                println!("You guessed the answer {}", secret_number);
                break;
            }
        }
    }

    println!("The answer is {}, you tried {} times", secret_number, tries);
}