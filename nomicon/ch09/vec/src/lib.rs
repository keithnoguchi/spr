//! [Vec]: Implementing std::vec::Vec from Scratch
//!
//! [vec]: https://doc.rust-lang.org/nomicon/vec/vec.html
use std::alloc::{alloc, handle_alloc_error, realloc, Layout};
use std::fmt::{self, Debug};
use std::mem::size_of;
use std::ptr::NonNull;

pub struct Vec<T> {
    ptr: NonNull<T>,
    cap: usize,
    _len: usize,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Debug for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec")
            .field("ptr", &self.ptr)
            .field("len", &self._len)
            .field("cap", &self.cap)
            .finish()
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        assert!(size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Self {
            ptr: NonNull::dangling(),
            _len: 0,
            cap: 0,
        }
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large",
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { realloc(old_ptr, old_layout, new_layout.size()) }
        };
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec;

    #[test]
    fn new() {
        let v = Vec::<u64>::new();
        assert_eq!(v.cap, 0);
        assert_eq!(v._len, 0);
    }

    #[test]
    fn grow() {
        let mut v = Vec::<u32>::new();
        v.grow();
        assert_eq!(v.cap, 1);
        assert_eq!(v._len, 0);
        v.grow();
        assert_eq!(v.cap, 2);
        assert_eq!(v._len, 0);
        v.grow();
        assert_eq!(v.cap, 4);
        assert_eq!(v._len, 0);
    }
}
