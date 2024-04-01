use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    //
    // let result = longest(&string1.as_str(), &string2[..]);
    // println!("The longest string is {}", result);

    // ------------------------------------------------------------
    // let novel2 = String::from("Call me Ishmael. Some years ago...");
    // let first_sentence = novel2.split('.').next().expect("Could not find a '.'");
    // let i = ImportantExcerpt {
    //     part: first_sentence,
    // };
    //
    // println!("i: {:?}", i);
    // ------------------------------------------------------------

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        &string1.as_str(),
        &string2[..],
        String::from("Announcement").as_str(),
    );
    println!("The longest string is {}", result);
}
