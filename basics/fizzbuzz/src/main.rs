// FizzBuzz: A program that prints numbers from 1 to 100, but for multiples of three, it prints "Fizz" instead of the number,
// and for the multiples of five, it prints "Buzz". For numbers that are multiples of both three and five, it prints "FizzBuzz".


fn main() {

    // bruteforce implementation
    for n in 1..=100{
        if n % 15 == 0{
            println!("FizzBuzz");
        } else if n % 3 == 0{
            println!("Fizz");
        } else if n % 5 == 0{
            println!("Buzz");
        } else {
            println!("{}",n);
        }
    }

    // match implementation

    for m in 1..=100{
        match m{
            m if m%15 == 0 => println!("FizzBuzz"), 
            m if m%5 == 0 => println!("Buzz"), 
            m if m%3 == 0 => println!("Fizz"),
            _ => println!("{}", m),
        }
    }
    
}
