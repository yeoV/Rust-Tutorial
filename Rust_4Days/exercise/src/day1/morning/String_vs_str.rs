/*
*   &str : immutable reference to a string slice / String : mutable string buffer
*/
fn main() {
    let s1: &str = "Hello!";
    println!("s1 : {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);

    println!("s2: {s2}");
}
