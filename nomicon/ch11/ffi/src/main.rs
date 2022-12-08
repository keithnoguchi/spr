//! FFI example
//!
//! As in [Rustnomicon].
//!
//! [rustnomicon]: https://doc.rust-lang.org/nomicon/ffi.html
use ffi::max_compressed_length;

fn main() {
    let x = max_compressed_length(100);
    println!("max compressed length of 100 byte buffer: {x}");
}
