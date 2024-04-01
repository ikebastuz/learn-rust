use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please input your guess");

        io::stdin().read_line(&mut guess).expect("Failed to guess!");

        let guess_num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let guess = match Guess::new(guess_num) {
        //     g => g,
        //     _ => continue,
        // };
        let guess = Guess::new(guess_num);

        println!("You guessed: {:?}", guess);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Win!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
