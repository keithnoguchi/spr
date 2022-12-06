//! [Vec]: Implementing std::vec::Vec from Scratch
//!
//! [vec]: https://doc.rust-lang.org/nomicon/vec/vec.html
use std::alloc::{self, Layout};
use std::fmt::{self, Debug};
use std::mem;
use std::ops::Deref;
use std::ptr::{self, NonNull};
use std::slice;
use tracing::{instrument, trace};

pub struct Vec<T> {
    buf: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.buf.as_ptr(), self.len) }
    }
}

impl<T> Drop for Vec<T> {
    #[instrument(name = "Vec::drop")]
    fn drop(&mut self) {
        if self.cap != 0 {
            while self.pop().is_some() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.buf.as_ptr() as *mut u8, layout);
            }
            trace!("dropped");
        }
    }
}

impl<T> Debug for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec")
            .field("buf", &self.buf)
            .field("len", &self.len)
            .field("cap", &self.cap)
            .finish()
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Self {
            buf: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, v: T) {
        if self.len == self.cap {
            self.grow()
        }
        unsafe {
            ptr::write(self.buf.as_ptr().add(self.len), v);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.buf.as_ptr().add(self.len))) }
        }
    }

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

        let new_buf = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_buf = self.buf.as_ptr() as *mut u8;
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::realloc(old_buf, old_layout, new_layout.size()) }
        };
        self.buf = match NonNull::new(new_buf as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
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
        assert_eq!(v.len, 0);
    }

    #[test]
    fn slice() {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.first(), Some(&1));
        assert_eq!(v.last(), Some(&2));
    }

    #[test]
    fn push() {
        let mut v = Vec::new();
        v.push(9);
        v.push(10);
        assert_eq!(v.len, 2);
        assert_eq!(v.cap, 2);
        v.push(1);
        assert_eq!(v.len, 3);
        assert_eq!(v.cap, 4);
    }

    #[test]
    fn pop() {
        let mut v = Vec::new();
        assert_eq!(v.pop(), None);
        v.push("first");
        v.push("second");
        assert_eq!(v.pop(), Some("second"));
        assert_eq!(v.pop(), Some("first"));
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);
        assert_eq!(v.len, 0);
        assert_eq!(v.cap, 2);
    }

    #[test]
    fn grow() {
        let mut v = Vec::<u32>::new();
        v.grow();
        assert_eq!(v.cap, 1);
        assert_eq!(v.len, 0);
        v.grow();
        assert_eq!(v.cap, 2);
        assert_eq!(v.len, 0);
        v.grow();
        assert_eq!(v.cap, 4);
        assert_eq!(v.len, 0);
    }
}
