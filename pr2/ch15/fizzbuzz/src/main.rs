use fizzbuzz::fizzbuzz;
use std::str::FromStr;

fn main() {
    let limit = std::env::args()
        .nth(1)
        .as_ref()
        .and_then(|n| usize::from_str(n).ok())
        .unwrap_or(30);
    for s in fizzbuzz().take(limit) {
        println!("{s}");
    }
}
