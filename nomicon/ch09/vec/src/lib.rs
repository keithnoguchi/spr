//! [Vec]: Implementing std::vec::Vec from Scratch
//!
//! [vec]: https://doc.rust-lang.org/nomicon/vec/vec.html
use std::fmt::{self, Debug};
use std::mem::size_of;
use std::ptr::NonNull;

pub struct Vec<T> {
    _ptr: NonNull<T>,
    _cap: usize,
    _len: usize,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Debug for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec")
            .field("ptr", &self._ptr)
            .field("len", &self._len)
            .field("cap", &self._cap)
            .finish()
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        assert!(size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Self {
            _ptr: NonNull::dangling(),
            _len: 0,
            _cap: 0,
        }
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::Vec;

    #[test]
    fn new() {
        let v = Vec::<u64>::new();
        assert_eq!(v._cap, 0);
        assert_eq!(v._len, 0);
    }
}
