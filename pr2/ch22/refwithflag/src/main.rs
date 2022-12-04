//! RefWithFlag: Reference with Packed Bool Flag
//!
//! *const T, a shared raw pointer, example demonstrated in
//! [Programming Rust, 2nd Edition], page 642.
//!
//! [programming rust, 2nd edition]: https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/
use refwithflag::RefWithFlag;

fn main() {
    let v = vec![10, 20, 30];
    let flagged = RefWithFlag::from((&v, true));
    let unflagged = RefWithFlag::from((&v, false));

    println!("flagged: {flagged:?}");
    println!("unflagged: {unflagged:?}");

    println!(
        "v[0]={}, v[1]={}, v[2]={}, v.len()={}",
        flagged.as_ref()[0],
        unflagged.as_ref()[1],
        flagged.as_ref()[2],
        unflagged.as_ref().len(),
    );
}
