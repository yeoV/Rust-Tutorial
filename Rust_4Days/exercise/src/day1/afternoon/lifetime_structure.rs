#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}");
}

fn main() {
    let text = String::from("The quick brown ...");
    let fox = Highlight(&text[4..7]);
    let dog = Highlight(&text[9..12]);

    // Highlight 함수가 사용하는 한 (&str) erase 불가
    // erase(text);
    println!("{fox:?}");
    println!("{dog:?}");
    erase(text);
}
