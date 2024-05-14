// Factorial Calculator: A program that calculates the factorial of a given number.

use std::io; // package to use input/output from the standard lib


fn factorial(x:i32) -> i32{

    if x ==1{
        1
    }
    else {
        return x*factorial(x-1);
    }

}


fn main() {
    println!("Factorial calculator, input a number:)\n");
    let mut inputline = String::new();
    io::stdin().read_line(&mut inputline).expect("Failed to read line!");
    let inputline: i32 = inputline.trim().parse().expect("Please enter a number!");
    println!("Factorial of {} is {}", inputline, factorial(inputline));


}

