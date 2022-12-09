//! Default trait excercise
use std::collections::HashSet;

fn main() {
    let squares = [2, 4, 6, 8, 10, 14, 16];
    let (power_of_two, invalid): (HashSet<i32>, HashSet<i32>) =
        squares.iter().partition(|&n| n & (n - 1) == 0);
    println!("power of two: {power_of_two:?}");
    println!("invalid: {invalid:?}");

    let (capped, uncapped): (String, String) = "Great Teacher Onizuka"
        .chars()
        .partition(|&c| c.is_uppercase());
    println!("capped: {capped}");
    println!("uncapped: {uncapped}");
}
