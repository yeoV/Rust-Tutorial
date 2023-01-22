use std::collections::HashMap;

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Hello".to_string(), 201);

    if page_counts.contains_key("Hello") {
        println!("World! {}", page_counts.len());
    }

    for book in ["Hello"] {
        match page_counts.get(book) {
            Some(count) => println!("{book} : {count} pages"),
            None => println!("{book} is unknown"),
        }
    }
}
