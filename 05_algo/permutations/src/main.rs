use std::io;

fn permutations(word: String) -> Vec<String> {
    println!("Word: {}, len: {}", word, word.len());
    if word.len() <= 1 {
        return vec![word];
    }

    let trimmed: String = word.chars().skip(1).collect();
    println!("Trimmed: {:?}", trimmed);

    let perms = permutations(trimmed);
    let first_char = word.chars().nth(0).unwrap();
    let mut result = Vec::new();

    for perm in &perms {
        for i in 0..&perms.len() + 1 {
            let front: String = perm.chars().take(i).collect();
            let rest: String = perm.chars().skip(i).collect();
            result.push(format!("{}{}{}", front, first_char, rest));
        }
    }

    result
}

fn main() {
    let mut inp = String::new();
    println!("Enter your word");

    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to take your input");

    let text = inp.trim().to_string();

    println!("Creating permutations of string: {}", text);

    let result = permutations(text);

    let mut to_print = result.clone();
    to_print.sort();
    to_print.dedup();

    println!("Permutations: \n{:?}", to_print);
}
