pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn capitalize_words_1(words: &[&str]) -> Vec<String> {
    words.to_owned().into_iter().map(capitalize_first).collect()
}

pub fn capitalize_words_2(words: &[&str]) -> String {
    words
        .to_owned()
        .into_iter()
        .map(capitalize_first)
        .collect::<String>()
}

fn main() {
    println!("Hello, world!");
    let v1: Vec<i32> = vec![1, 2, 3];

    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("got: {}", val);
    }

    for val in &v1 {
        println!("gotv2: {}", val);
    }

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    let words = vec!["hello", "world"];
    let test1 = capitalize_words_1(&words);
    let test2 = capitalize_words_2(&words);

    println!("test1: {:?}", test1);
    println!("test2: {:?}", test2);
}
