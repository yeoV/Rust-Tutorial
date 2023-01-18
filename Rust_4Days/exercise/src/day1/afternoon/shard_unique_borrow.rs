fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a; // a가 b 에게 immutable로 빌려줌

    // 항당받은 변수 사용 -> Non-lexical lifetime
    println!("b : {b}");

    {
        let c: &mut i32 = &mut a; // c 에게 동시에 mutable로 빌려줌
        *c = 20;
    }

    println!("a : {a}");
}
