fn main() {
    let five = Box::new(5);
    // * 포인트 호출 시, Deref.
    println!("five : {}", *five);
}
