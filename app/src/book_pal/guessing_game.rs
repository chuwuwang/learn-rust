extern crate rand;

use std::cmp::Ordering;
use std::io;

use rand::Rng;

pub fn guess() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (_i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..1];
        }
    }
    return &s[..]
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

