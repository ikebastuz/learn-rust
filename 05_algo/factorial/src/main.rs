use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the number to calculate factorial");

    io::stdin().read_line(&mut input).expect("Try again");

    let input: u32 = input.trim().parse().expect("Not a number");
    let result = factorial(input);

    println!("Result: {result}");
}

fn factorial(input: u32) -> u32 {
    if input <= 1 {
        return 1;
    }

    input * factorial(input - 1)
}
