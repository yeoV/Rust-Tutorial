// Mutable variable binding
fn main() {
    let mut x: i32 = 127; // mut : mutable, i8 -> 8bit 정수까지 표현 가능
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!("-> {x}");
    }
    println!();
}
