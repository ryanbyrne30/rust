use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number! Hint it's {secret}");
    println!("What is your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {guess}");

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You win!"),
    }
}
