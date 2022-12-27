use futures_lite::future;
use spawn_blocking::spawn_blocking;
use std::thread;
use std::time::Duration;

fn main() {
    tracing_subscriber::fmt::init();

    let hello = future::block_on(spawn_blocking(|| {
        thread::sleep(Duration::from_millis(10));

        "hello, async world!"
    }));

    println!("{hello}");
}
