/*
Copying and cloning are not the same thing:

- Copying refers to bitwise copies of memory regions and does not work on arbitrary objects.
- Copying does not allow for custom logic (unlike copy constructors in C++).
- Cloning is a more general operation and also allows for custom behavior by implementing the Clone trait.
- Copying does not work on types that implement the Drop trait.
*/

// #[derive(Copy, Clone, Debug)]
#[derive(Clone, Debug)]
struct Point(i32, i32);

fn main() {
    // 아래 형식들은 copy 가능 타입
    let x = 42;
    let y = x;
    println!("x : {x}");
    println!("y : {y}");

    let p1 = Point(3, 4);
    let mut p2 = p1.clone(); // copy semantics

    println!("p1 : {p1:?}");
    p2.0 = 10;
    println!("p2 : {p2:?}");
}
