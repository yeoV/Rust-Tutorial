pub fn without_type() {
    let my_func = |x| x + 1;
    println!("Without Type : {}", my_func(3));
}
pub fn with_type() {
    let my_func = |x: i32| -> i32 { x + 1 };
    println!("With Type : {}", my_func(5))
}
