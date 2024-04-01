use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the number to calculate factorial");
    io::stdin().read_line(&mut input).expect("Try again");

    let input: u32 = input.trim().parse().expect("Not a number");

    let sequence = fib(input);
    println!("Result: {sequence}");
}

fn fib(input: u32) -> String {
    if input == 0 {
        return "".to_string();
    } else if input == 1 {
        return "0".to_string();
    } else if input == 2 {
        return "0 1".to_string();
    }

    let mut output_string = String::from("0 1");

    let mut a = 0;
    let mut b = 1;

    for _x in 1..input - 1 {
        let next = a + b;
        a = b;
        b = next;

        output_string = format!("{} {}", output_string, b);

        // let append = format!(" {b}");
        // output_string.push_str(&append);

        // output_string = [output_string, b.to_string()].join(" ");
    }

    output_string.trim().to_string()
}

#[test]
fn it_calculates_sequence() {
    assert!(fib(5) == "0 1 1 2 3");
    assert!(fib(10) == "0 1 1 2 3 5 8 13 21 34");
    assert!(fib(0) == "");
}
