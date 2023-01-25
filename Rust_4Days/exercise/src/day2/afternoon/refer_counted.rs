use std::rc::Rc;

// C++ shared_ptr 과 비슷 -> 더 이상 사용하지 않는 경우 자동 free
fn main() {
    let mut a = Rc::new(10);
    // create point same allocation, increase refer count
    let mut b = a.clone();

    println!("a: {a}");
    println!("b: {b}");
}
