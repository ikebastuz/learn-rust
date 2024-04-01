use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let v = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *v);
    print!("!");

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    hello(&String::from("Test"));
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
