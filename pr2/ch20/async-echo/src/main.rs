use async_echo::run;
use futures_lite::future;

const ADDR: &str = "localhost:8080";

fn main() {
    let addr = std::env::args().nth(1).unwrap_or_else(|| ADDR.to_string());

    if let Err(e) = future::block_on(run(addr)) {
        eprintln!("{e}");
    }
}
