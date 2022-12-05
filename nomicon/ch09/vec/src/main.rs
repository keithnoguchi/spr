//! [Vec]: Implementing std::vec::Vec from Scratch
//!
//! [vec]: https://doc.rust-lang.org/nomicon/vec/vec.html
use vec::Vec;

fn main() {
    let v = Vec::<u32>::new();

    println!("{v:?}");
}
