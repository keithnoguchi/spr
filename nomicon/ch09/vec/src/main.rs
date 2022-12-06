//! [Vec]: Implementing std::vec::Vec from Scratch
//!
//! [vec]: https://doc.rust-lang.org/nomicon/vec/vec.html
use vec::Vec;

fn main() {
    tracing_subscriber::fmt::init();
    let mut v = Vec::new();
    v.push("first".to_string());
    v.push("second".to_string());
    v.push("third".to_string());
    println!("{v:?}");
    println!("first()={:?}", v.first());
    println!("last()={:?}", v.last());
    for v in v.iter_mut() {
        *v = v.to_uppercase();
    }
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
}
