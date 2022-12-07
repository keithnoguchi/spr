//! [Vec]: Implementing std::vec::Vec from Scratch
//!
//! [vec]: https://doc.rust-lang.org/nomicon/vec/vec.html
use vec::Vec;

fn main() {
    tracing_subscriber::fmt::init();
    let mut vec = Vec::new();
    vec.push("first".to_string());
    vec.push("second".to_string());
    vec.push("third".to_string());
    for v in vec.iter() {
        println!("{v}");
    }
    for v in vec.drain() {
        println!("{v}");
    }
    println!("v.len()={}", vec.len());
}
