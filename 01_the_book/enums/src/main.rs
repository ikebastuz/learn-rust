#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    WTF,
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let _loopback = IpAddr::V6(String::from("::1"));
    println!("{:#?}", home);

    let c1 = Coin::Penny;
    let c1_price = value_in_cents(c1);
    println!("c1 price: {}", c1_price);

    let c2 = Coin::Dime;
    let c2_price = value_in_cents(c2);
    println!("c2 price: {}", c2_price);

    let c3 = Coin::WTF;
    let c3_price = value_in_cents(c3);
    println!("c3 price: {}", c3_price);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => 100,
    }
}
