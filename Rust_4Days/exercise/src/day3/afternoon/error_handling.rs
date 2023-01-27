/*
- 런타임 환경에서 에러 발생 가능
- Use non-packing API를 사용할 것. 예) Vec::Get

*/
use std::panic;

fn main() {
    let v = vec![10, 20, 30];
    println!("v[100] : {}", v[100]);

    // panic을 이용한 에러핸들링
    let result = panic::catch_unwind(|| {
        panic!("Oh panic!");
    });
    assert!(result.is_err());
}
