// Prime Number Checker: A program that checks whether a given number is prime or not.



use std::io; // package to use input/output from the standard lib



fn isprime(x:i32) -> bool{
    for n in (2..x).rev(){
        println!("Trying division of {} and {}",x,n);
        if x%n==0{
            println!("Hit at n = {}",n);
            return false;
         
        }
        
    }
    return true;
}


fn isprimetrialdivision(x:i32)->bool{ // Trial division tests if a number is prime by checking divisibility up to its square root.
    let sqrrtoottest = (x as f64).sqrt().floor() as i32;
    for n in (2..=sqrrtoottest).rev(){
        println!("Trying division of {} and {}",x,n);
        if x%n==0{
            println!("Hit at n = {}",n);
            return false;
         
        }
        
    }
    return true;
}



fn main() {

    println!("Input a number to check wether is prime or not! \n >>");
    let mut inputline = String::new();
    io::stdin().read_line(&mut inputline).expect("Failed to read line!");
    let inputline: i32 = inputline.trim().parse().expect("Please enter a number!");

    println!("{}",isprime(inputline));
    println!("{}",isprimetrialdivision(inputline)); //less comparisons so its more optimal

}
