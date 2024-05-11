// Guessing Game: A game where the computer selects a random number and the user has to guess it.

use std::io; // package to use input/output from the standard lib
use rand::Rng; // package to generate random numbers (random number generator)

fn main() {


    let secret_number = rand::thread_rng().gen_range(0..11);
    println!("Hi this test program will generate a random number and you have to guess it!\n Come on, guess a number between 1 and 10!\n My guess: \t");
    let mut guess = String::new(); // create a new object to store a mutable variable called 'guess' 
    io::stdin().read_line(&mut guess) // reading input and storing in guess variable
        .expect("Failed to read line!"); // error handling
    println!("You guessed {}", guess); 
    println!("Shhh, the random number was:{}", secret_number);

    

}
