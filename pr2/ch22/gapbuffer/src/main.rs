//! GapBuffer
//!
//! Example demonstrated in [Programming Rust] 2nd Edition, page 651.
use gapbuffer::GapBuffer;
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();

    let mut buf = GapBuffer::new();
    info!("After new(): {buf:#?}");
    let text = "Load of the Rings";
    buf.insert_iter(text.chars());
    info!("After insert_iter({text:?}): {buf:#?}");
    let position = 12;
    buf.set_position(position);
    info!("After set_position({position}): {buf:#?}");
    let text = "Onion ";
    buf.insert_iter(text.chars());
    info!("After insert_iter({text:?}): {buf:#?}");

    println!("buf[12]={:?}", buf.get(12).unwrap());
}
