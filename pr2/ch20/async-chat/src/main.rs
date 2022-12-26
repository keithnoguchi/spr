use async_chat::server;
use futures_lite::future;

const ADDR: &str = "localhost:8080";

fn main() {
    tracing_subscriber::fmt::init();
    let addr = std::env::args().nth(1).unwrap_or_else(|| ADDR.to_string());

    if let Err(e) = future::block_on(server(addr)) {
        eprintln!("server: {e}");
    }
}
