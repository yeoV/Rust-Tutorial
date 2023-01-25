mod foo {
    pub fn do_something() {
        println!("In the foo module");
    }
}

mod bar {
    pub fn do_something() {
        println!("In the bar module");
    }

    pub mod inner {
        fn private() {
            println!("bar::inner::private");
        }
        pub fn public() {
            println!("bar::inner::public");
            private();
        }
    }
}

fn main() {
    foo::do_something();
    bar::do_something();
    bar::inner::public();
}
