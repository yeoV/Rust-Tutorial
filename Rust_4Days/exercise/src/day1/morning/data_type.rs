/*
    List Array : [T; N]
    Tuple      : (), (T,), (T1, T2), ...
*/
fn main() {
    let _str: &str = "Hello world!";
    println!("String Type &str : {}", _str);
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("Array List a : {a:#?}");
    // a : [42, 42, 42, 42, 42, 0, 42, 42, 42, 42]
    // {:#} -> pretty

    let mut t: (i8, bool) = (7, true);
    println!("1st index : {}", t.0);
    println!("2st index : {}", t.1);
    t.0 = 19;
    println!("Changed 1st index : {}", t.0);
}
