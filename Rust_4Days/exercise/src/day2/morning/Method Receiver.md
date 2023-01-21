## Method Receiver

The `&self` above indicates that the method borrows the object immutably.
There are other possible receivers for a method:

- `&self` : caller 로 부터 object를 borrow 해줌. immutable
- `&mut self` : caller 로 부터 object를 borrow 해줌. mutable
- `self` : takes ownership of the object. caller로 부터 값 복사.
    method return 시, object drop
- None : static method on struct. 


이 외, special warpper type 존재
- https://doc.rust-lang.org/reference/special-types-and-traits.html


## ?? Onwership vs Borrow 치이?
