use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the number");
    io::stdin().read_line(&mut input).expect("Try again");

    let size = input.trim().parse().expect("Not a number");

    for row in 0..size {
        println!("{}", line(size - row));
    }
}

fn line(n: u64) -> String {
    let mut res = String::new();

    for _ in 0..n {
        res.push_str("*");
    }

    res
}
