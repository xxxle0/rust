// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

// I AM NOT DONE

macro_rules! my_macro {
    ($name: expr) => {
        return "Hello " + $name;
    };
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}

