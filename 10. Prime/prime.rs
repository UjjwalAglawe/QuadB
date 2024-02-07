use std::io;
use std::io::Write;

fn prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    
    let int_input: u64 = input.trim().parse().expect("Please enter a valid number");

    if(prime(int_input)) {
        println!("{} is a prime number", int_input);
    } else {
        println!("{} is not a prime number", int_input);
    };
}
