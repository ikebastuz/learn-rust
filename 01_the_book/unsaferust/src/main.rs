fn main() {
    println!("Hello, world!");

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // let address = 0x01234usize;
    // let r = address as *const i32;
    //

    println!("r1 is {:?}", r1);
    println!("r2 is {:?}", r2);

    unsafe {
        println!("r1 is {:?}", *r1);
        println!("r2 is {:?}", *r2);
    }
}
