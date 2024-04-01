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
}
