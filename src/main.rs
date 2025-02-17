use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Rusty Guess!");
    println!("The game of guessing a number between 1 and 100.\n\n");

    let secret_number: i32 = rand::rng().random_range(1..=100);

    loop {
        println!("Please enter your guess:");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the user input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!!!");
                break;
            }
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too Small"),
        }

        println!("You guessed {guess}");
    }
}
