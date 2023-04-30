use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        // mutable string init from base struct String
        // rust has "two kinds" of strings: String, &str (string slice)
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // so here we are parsing a string to a number
        // let _guess_u32: u32 = guess.trim().parse().expect("Please type a number!");
        // this could be done more securely using ::<> ('turbofish') syntax:
        // since type inference makes it more finicky to use parse()
        // ------------------------
        // This here is manual error handling, in case you want more specific logic or errors,
        // rather than just type checking. In this case we're just returning to the loop, so
        // there's no panic!
        // This is part of the flow controls of rust
        let secure_guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(value) => {
                println!("Please type a number doofus. Error: {value}");
                continue;
            }
        };

        println!("You guessed: {guess}");
        // pattern matching to rusts powerful enums
        match secure_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small pp!"),
            Ordering::Greater => println!("Too big pp!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
