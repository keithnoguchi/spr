//! let's play with the static variable.
use statics::{f, STASH};

fn main() {
    static V: i32 = 1024;
    println!("&V={:p}, V={}", &V, V);
    debug(&V, "before f(&V)");
    f(&V);
    debug(&V, "after f(&V)");
}

fn debug(v: &i32, msg: &str) {
    print!("{msg}: ");
    print!("&V={:p}, V={} ", v, v);
    unsafe {
        println!("STASH={:p}, STASH={}, *STASH={}", STASH, STASH, *STASH);
    }
}
