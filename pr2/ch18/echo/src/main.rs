fn main() {
    if let Err(e) = echo::server("127.0.0.1:8888") {
        eprintln!("{e}");
    }
}
