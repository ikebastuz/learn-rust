use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
        println!("Vector: {:?}", v);
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
