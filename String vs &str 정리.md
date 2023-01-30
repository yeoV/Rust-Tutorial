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
