/*
* i8 타입의 x를 i8에서 i16으로 변경하는 법
* 1. i16::from(x)
* 2. x.into()
*/
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
}
