fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name); // name 변수 ownership 제공 (&name)

    // 함수 재호출 시, 첫번쨰 call 에 name.clone() 했기에 에러 발생
    // say_hello(name);
}
