use std::collections::HashMap;

fn main() {
    // vec();
    // string();
    hashmaps();
}

fn hashmaps() {
    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // scores.insert(String::from("Green"), 150);
    // scores.insert(String::from("Green"), 250);
    //
    // let team_name = String::from("Blue2");
    // let default_score = 0;
    // let score = scores.get(&team_name).unwrap_or(&default_score);
    //
    // println!("{score}");
    //
    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }
    //
    // println!("{:?}", scores);
    // --------------------------------------------
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn _string() {
    // let mut s = String::new();
    // let data = "initial_contents";
    // let s2 = data.to_string();
    //
    // s.push_str("Hello world");
    // println!("{} {} {}", data, s2, s);
    // --------------------------------------------
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    //
    // println!("{} {}", s2, s3);
    // --------------------------------------------

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    //
    // let s = format!("{s1}-{s2}-{s3}");
    // println!("s: {}", s);
    // --------------------------------------------
    //
    let hello = "Здравствуйте";

    let s = &hello[0..2];

    println!("{}", s);

    for c in hello.chars() {
        print!("{c}");
    }
    for b in hello.bytes() {
        print!("{b}");
    }
}

fn _vec() {
    let mut v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    println!("third v1: {}", third);

    let third2: Option<&i32> = v.get(5);
    match third2 {
        Some(num) => println!("third v2: {}", num),
        None => println!("third v2 is None"),
    }

    for i in &v {
        println!("i: {i}");
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("i: {i}");
    }
}
