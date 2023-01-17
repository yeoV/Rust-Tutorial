/*
* Move Semantics
* - s1 변수가 s2에 할당
* - s1 변수의 ownership이 s2로 넘어가, s1은 접근 불가능.
* - Exactly하게 하나의 변수에는 하나의 값만 할당 가능
* - Heap 에 할당된 Hello String을 s2가 할당
*/

fn main() {
    let s1: String = String::from("Hello");
    let s2: String = s1;

    println!("s2 : {s2}");
    // println!("s1 : {s1}");
}
