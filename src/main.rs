use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Rusty Guess!");
    println!("The game of guessing a number between 1 and 100.\n\n");

    let secret_number: i32 = rand::rng().random_range(1..=100);

    println!("Generated secret number {secret_number}");

    println!("Please enter your guess:");
    let mut guess: String = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read the user input");

    println!("You guessed {guess}");
}