//! Chapter 5: References
use artists::Table;

fn main() {
    let mut table = Table::new();
    table.insert("Alice Noguchi", "Safe Programming in Rust");
    table.insert("Bob Noguchi", "Photography in Action");
    table.insert("Alice Noguchi", "Atomic Programming in Rust");
    table.insert("Alice Noguchi", "Concurrent Programming in Rust");
    table.insert("Chris Noguchi", "Spirituality in Action");
    table.insert("Alice Noguchi", "Blockchain Programming in Rust");
    println!("{table}");

    println!("Let's sort the works");
    table.sort_works();
    println!("{table}");
}
