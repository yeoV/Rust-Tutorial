# String vs &str 정리

- String : Heap 영역에 할당. mutable
- str : immutable fixed-length, somewhere in-memory

## 한줄요약

Prefer `&str` as a function parameter or if you want a read-only view of a `string`;

String when you want to own and mutate a string.

### String

- 데이터 타입 자체는 stack, 실제 데이터는 heap에 저장
- Owned Value
- semantically equivalent to `Stringbuffer` in JAVA

``` rust
let mut s = String::from("Hello, Rust!");
println!("{}", s.capacity()); // prints 12
s.push_str("Here I come!");
println!("{}", s.len()); // prints 24
```

### &str

- You can only ever interact with str as a borrowed type aka &str.
- This is called a string slice, an immutable view of a string. This is the preferred way to pass strings around, as we shall see.

```rust
fn main() {
    let s = String::from("Hello, Rust!");
    foo(&s);
}

fn foo(s: &str) {
    println!("{}", s);
}
```

- In the above example, foo() can take either string slices or borrowed Strings, which is super convenient.

***

## Method parameter between `&str` and `&string`

```rust

// 1. data: &String
fn print_data(data: &String){
    println!("printing my data {}", data);
}

// 2. data: &str
fn print_data(data: &str){
    println!("printing my data {}", data);
}

```

### Option 1

- `&String` 타입은 String 타입에 대해서 매개변수로 받는다.
- print_data 의 매개변수로 넘겨도 my_string은 heap에 ownership을  가지고 있다.

```rust
fn main() {
    let my_string = String::from("Understanding the String concept?");
 
    print_data(&my_string);

    print!("printing inside main {}", my_string); 
}

fn  print_data(data: &String) {
    println!("printing my data {} ", data);
}
```

- my_string 을 str 형식으로 변경하여 넘길 수 있다.
- `&my_str.to_string()` 은 여전히 data를 &로 넘겨야 함.
- 하지만 데이터는 heap 영역에 String으로써 할당

```rust
let my_str = "This is a str";
print_data(&my_str.to_string());
```

### Option 2

```rust
fn main() {
    let my_string = String::from("Understanding the String concept?");
 
    print_data(&my_string);

    print!("printing inside main {}", my_string); 
}

fn  print_data(data: &str) {
    println!("printing my data {} ", data);
}
```

- print_data 로 my_string이 borrow 됨
- **일반적으로 type `&str`은 다른 `String`에 복사본을 만들 필요가 없기 때문에 함수 매개 변수로 전달될 때 유용합니다.**
