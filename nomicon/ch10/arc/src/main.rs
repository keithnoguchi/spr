//! std::sync::Arc from Scratch
//!
//! As in the [rustnomicon].
//!
//! [rustnomicon]: https://doc.rust-lang.org/nomicon/arc-mutex/
use arc::Arc;

fn main() {
    tracing_subscriber::fmt::init();

    let arc = Arc::new("This is a test");
    let arc2 = arc.clone();

    println!("{}", *arc2);
}
