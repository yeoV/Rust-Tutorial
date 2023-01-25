fn main() {
    let mut x = 10;
    x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };

    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name : {value}");
    } else {
        println!("Missing name?");
    }

    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
    // loop -> endless loop
    loop {
        print!("hello");
        break;
    }

    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}
