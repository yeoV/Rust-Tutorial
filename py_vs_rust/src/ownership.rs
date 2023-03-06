/***********************
 * 에러 발생하는 경우
 * ********************* */
fn error_dummy(x: String) {
    println!("{}", x);
}

fn borrow() {
    let x = String::from("Hello");
    error_dummy(x);
    println!("{}", x); // This code raise error
}
/***********************
 * Borrow의 경우
 * ********************* */

fn borrow_dummy(y: &String) {
    println!("{}", y);
}

fn reference_borrow() {
    let x = String::from("Borrow");
    borrow_dummy(&x);
    println!("{}", x); // Run well
}
/***********************
 * 가변 참조 / 다중 레퍼런스
 * -> 가변 참조의 경우, "한번의 하나의 소유자만 존재할 수 있음"
 * -> 단순 참조하는 다중 레퍼런스는 가능
 * ********************* */

fn mutable_dummy(y: &mut String) {
    y.push("World!");
    println!("{}", y);
}

fn reference_mutable() {
    let mut x = String.from("Hello");
    mutable_dummy(x);
    println!("{}", x)
}

fn main() {
    // borrow(); // raise error
    reference_borrow();
    reference_mutable();
}
