// Temperature Converter: A program that converts temperatures between Celsius and Fahrenheit.
use std::io; // package to use input/output from the standard lib

fn ctof(x:f64)-> f64{
    (x*1.8) +32.0
}

fn ftoc(x:f64)-> f64{
    (x-32.0)/1.8
}

fn main() {
    
    loop{
        println!("Select operation (1: C -> F, 2: F -> C, 3: quit)\n");
        let mut inputline = String::new();
        io::stdin().read_line(&mut inputline).expect("Failed to read line!");
        let inputline: i32 = inputline.trim().parse().expect("Please enter a number!");
        
        if inputline == 1{
            println!("Input temperature in C\n");
            let mut c = String::new();
            io::stdin().read_line(&mut c).expect("Failed to read line!");
            let c: f64 = c.trim().parse().expect("Please enter a number!");
            println!("{}C to F is {}", c , ctof(c));
        } else if inputline==2{
            println!("Input temperature in F\n");
            let mut f = String::new();
            io::stdin().read_line(&mut f).expect("Failed to read line!");
            let f: f64 = f.trim().parse().expect("Please enter a number!");
            println!("{}F to C is {}", f , ftoc(f));


        } else if inputline==3{
            break;
        } else {
            println!("Wrong operation!");
        }

    }

}
