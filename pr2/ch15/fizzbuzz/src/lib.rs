//! Iterator Adaptor
use std::iter::{once, repeat};

pub fn fizzbuzz() -> impl Iterator<Item = String> {
    let fizz = repeat("").take(2).chain(once("fizz")).cycle();
    let buzz = repeat("").take(4).chain(once("buzz")).cycle();
    (1..).zip(fizz.zip(buzz)).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    })
}
