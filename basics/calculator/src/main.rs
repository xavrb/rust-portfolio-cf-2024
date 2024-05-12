// Calculator: A basic calculator program that performs arithmetic operations 
//(addition, subtraction, multiplication, division) based on user input.

use std::io; // package to use input/output from the standard lib
//use::std::cmp::Ordering;



fn mysum(x: f32, y: f32) -> f32{
    x+y
}

fn mysubst(x: f32, y: f32) -> f32{
    x-y
}

fn mymult(x: f32, y: f32) -> f32{
    x*y
}

fn mydiv(x: f32, y: f32) -> f32{
    x/y
}


fn main() {
    println!("Hi this program works a a simple calculator for the 4 basic operations\n:");

    let mut a = String::new();
    let mut b = String::new();
    println!("Input number a:\t");
    io::stdin().read_line(&mut a).expect("Failed to read line!");
    let a: f32 = a.trim().parse().expect("Please enter a number!");
    println!("Input number b:\t");
    io::stdin().read_line(&mut b).expect("Failed to read line!");
    let b: f32 = b.trim().parse().expect("Please enter a number!");

    println!("I got numbers {} and {}", a,b);
    println!("Sum of numbers is: {}", mysum(a,b));
    println!("Substraction of numbers is: {}", mysubst(a,b));
    println!("Multiplication of numbers is: {}", mymult(a,b));
    println!("Division of numbers is: {}", mydiv(a,b));
}
