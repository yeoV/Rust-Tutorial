// ref
fn main() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x : {x}");

    // Rust는 갹 value 들에 대한 life cycle 관리
    let ref_x2: &i32;
    {
        let x: i32 = 10;
        ref_x2 = &x;
    }
    println!("ref_x2 : {ref_x2}");
}
