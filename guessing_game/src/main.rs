extern crate rand;

use std::cmp::Ordering;
use std::io;
use std::io::Write;
use rand::Rng;

fn main() {

    println!("Guess The Number!\n");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("I have chosen a secret number.");

    loop {

        print!("Enter your guess => ");

        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Unable to Read Input.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
            Ordering::Greater => println!("Too Big!"),
        }
    }
}
