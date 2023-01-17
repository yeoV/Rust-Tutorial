fn main() {
    let a = 10;
    println!("a : {a}");
    {
        let a = "hello";
        println!("inner scope a : {a}");

        let a = true;
        println!("shadowed in inner scope a : {a}");
    }
    println!("after: {a}");
}
