// Guessing Game: A game where the computer selects a random number and the user has to guess it.

use std::io; // package to use input/output from the standard lib
use::std::cmp::Ordering;
use rand::Rng; // package to generate random numbers (random number generator)


fn main() {
    let random_number = rand::thread_rng().gen_range(1..=10);
    println!("Hi this test program will generate a random number and you have to guess it!\n Come on, guess a number between 1 and 10!");

    let mut inputline = String::new();
    io::stdin().read_line(&mut inputline).expect("Failed to read line!");
    let inputline: i32 = inputline.trim().parse().expect("Please enter a number!");

    println!("You guessed {}", inputline);
    println!("Shhh, the random number was: {}", random_number);

    match inputline.cmp(&random_number) {
        Ordering::Less => println!("Hmmm, that's so low. Think greater!"),
        Ordering::Greater => println!("Hmm, that's so high. Think smaller!"),
        Ordering::Equal => println!("You win this time! Your ego is writing checks your body can't cash!")
    }
}

