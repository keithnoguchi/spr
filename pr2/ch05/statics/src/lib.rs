//! let's play with the static variable.
pub static mut STASH: &i32 = &1_000_000_000;

pub fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}
