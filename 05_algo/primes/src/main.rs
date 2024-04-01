use std::io;

use primes::primes;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    println!("Enter upper limit");

    io::stdin().read_line(&mut input).expect("Try again");

    let input: usize = input.trim().parse().expect("Not a number");

    let (a, b) = primes(input);

    println!("Count: {}, Max: {}", a, b);
}
