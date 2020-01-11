// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    // this is arguments
    call_me(3);
}

// this is parameters
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
