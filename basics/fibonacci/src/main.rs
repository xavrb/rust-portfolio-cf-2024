// Fibonacci Sequence: A program that generates the Fibonacci sequence
// up to a certain number of terms specified by the user.
use std::io; // package to use input/output from the standard lib

fn fibo(x: u64) -> u64{
    if x == 0{
        0
    } else if x == 1{
        1
    } else{
        fibo(x-1) + fibo(x-2)
    }
}


fn main() {

    println!("Fibonacci number to calculate: \n >>");
    let mut inputline = String::new();
    io::stdin().read_line(&mut inputline).expect("Failed to read line!");
    let inputline: u64 = inputline.trim().parse().expect("Please enter a number!");


    for n in 1..inputline{
        println!("Fibo number is: {}",fibo(n))

    }


}
