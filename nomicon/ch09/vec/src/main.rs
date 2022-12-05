//! [Vec]: Implementing std::vec::Vec from Scratch
//!
//! [vec]: https://doc.rust-lang.org/nomicon/vec/vec.html
use vec::Vec;

fn main() {
    tracing_subscriber::fmt::init();
    let mut v = Vec::new();
    v.push("first");
    v.push("second");
    v.push("third");
    println!("{v:?}");
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
}
