#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 생성자
impl Person {
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}

struct Point(i32, i32);

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };

    println!("{} is {} years old", peter.name, peter.age);

    // Tuple struct
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);

    // Field Shorthand Syntax
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
}
