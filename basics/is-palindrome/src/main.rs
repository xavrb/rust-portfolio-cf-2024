// Palindrome Checker: A program that checks whether a given string is a palindrome or not.
use std::io; // package to use input/output from the standard lib

fn ispal(x: &String) -> bool{
    
    let mystr = x.trim();
    let mut aux = 0;
    let mut auy = mystr.len()-1;
    let mut result = false;
    for _n in 0..(mystr.len()/2){
        if mystr.chars().nth(auy) == mystr.chars().nth(aux){
            result = true;
        } else{
            result = false;
        }
        aux = aux +1;
        auy = auy -1;
    }
    return result;
}


fn main() {
    println!("Lets check for a palindrome, input a word:\n");
    let mut inputline = String::new();
    io::stdin().read_line(&mut inputline).expect("Failed to read line!");
    println!("{}", ispal(&inputline));


}
