// A simple Rust program to calculate the factorial of a number

use std::io;

fn main() {
    println!("Enter a number to calculate its factorial:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let num: u64 = input.trim().parse()
        .expect("Please enter a valid number");

    let factorial = calculate_factorial(num);

    println!("The factorial of {} is: {}", num, factorial);
}

fn calculate_factorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    n * calculate_factorial(n - 1)
}
