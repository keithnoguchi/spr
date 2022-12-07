//! [Vec]: Implementing std::vec::Vec from Scratch
//!
//! [vec]: https://doc.rust-lang.org/nomicon/vec/vec.html
use std::alloc::{self, Layout};
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};
use std::slice;
use tracing::{instrument, trace};

pub struct Vec<T> {
    buf: Buf<T>,
    len: usize,
}

pub struct IntoIter<T> {
    // just a placeholder for the ownership/drop.
    iter: BufIter<T>,
    _buf: Buf<T>,
}

impl<T> Drop for IntoIter<T> {
    #[instrument(name = "IntoIter::drop")]
    fn drop(&mut self) {
        for _ in &mut *self {}
        trace!("dropped")
    }
}

impl<T> Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntoIter")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let iter = BufIter::new(&self);
            let _buf = ptr::read(&self.buf);
            mem::forget(self);
            Self::IntoIter { _buf, iter }
        }
    }
}

pub struct Drain<'a, T: 'a> {
    iter: BufIter<T>,
    vec: PhantomData<&'a mut Vec<T>>,
}

impl<'a, T> Drop for Drain<'a, T> {
    #[instrument(name = "Drain::drop")]
    fn drop(&mut self) {
        for _ in self {}
        trace!("dropped");
    }
}

impl<'a, T> Debug for Drain<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drain").field("iter", &self.iter).finish()
    }
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Drop for Vec<T> {
    #[instrument(name = "Vec::drop")]
    fn drop(&mut self) {
        while self.pop().is_some() {}
        trace!("dropped");
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self {
            buf: Buf::default(),
            len: 0,
        }
    }
}

impl<T> Debug for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec")
            .field("buf", &self.buf)
            .field("len", &self.len)
            .finish()
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn drain(&mut self) -> Drain<T> {
        unsafe {
            let iter = BufIter::new(self);
            self.len = 0;
            Drain {
                iter,
                vec: PhantomData,
            }
        }
    }

    pub fn insert(&mut self, index: usize, v: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap() {
            self.buf.grow()
        }
        unsafe {
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr().add(index), v);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    pub fn push(&mut self, v: T) {
        if self.len == self.cap() {
            self.buf.grow()
        }
        unsafe {
            ptr::write(self.ptr().add(self.len), v);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }

    #[inline]
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    #[inline]
    fn cap(&self) -> usize {
        self.buf.cap
    }
}

struct Buf<T> {
    ptr: NonNull<T>,
    cap: usize,
}

struct BufIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> Debug for BufIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufIter")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl<T> Iterator for BufIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.add(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for BufIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> BufIter<T> {
    unsafe fn new(slice: &[T]) -> Self {
        Self {
            start: slice.as_ptr(),
            end: if slice.is_empty() {
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            },
        }
    }
}

unsafe impl<T> Send for Buf<T> {}
unsafe impl<T> Sync for Buf<T> {}

impl<T> Drop for Buf<T> {
    #[instrument(name = "Buf::drop")]
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
            trace!("dropped");
        }
    }
}

impl<T> Default for Buf<T> {
    fn default() -> Self {
        assert!(mem::size_of::<T>() != 0, "TODO: Implement ZST support");
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }
}

impl<T> Debug for Buf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Buf")
            .field("ptr", &self.ptr)
            .field("cap", &self.cap)
            .finish()
    }
}

impl<T> Buf<T> {
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
            let old_buf = self.ptr.as_ptr() as *mut u8;
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::realloc(old_buf, old_layout, new_layout.size()) }
        };
        self.ptr = match NonNull::new(new_buf as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

#[cfg(test)]
mod tests {
    use super::{Buf, Vec};

    #[test]
    fn drain() {
        let test = "Load of the Ring";
        let mut v = Vec::new();
        for c in test.chars() {
            v.push(c);
        }
        let mut drain = v.drain();
        assert_eq!(drain.next(), Some('L'));
        assert_eq!(drain.next(), Some('o'));
        drop(drain);
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn into_iter_next_back() {
        let test = "Load of the Ring";
        let mut v = Vec::new();
        for c in test.chars() {
            v.push(c);
        }
        let mut iter = v.into_iter();
        assert_eq!(iter.next_back(), Some('g'));
        assert_eq!(iter.next_back(), Some('n'));
        assert_eq!(iter.next_back(), Some('i'));
        assert_eq!(iter.next_back(), Some('R'));
    }

    #[test]
    fn into_iter_next() {
        let test = "Load of the Ring";
        let mut v = Vec::new();
        for c in test.chars() {
            v.push(c);
        }
        let mut iter = v.into_iter();
        assert_eq!(iter.next(), Some('L'));
        assert_eq!(iter.next(), Some('o'));
        assert_eq!(iter.next(), Some('a'));
        assert_eq!(iter.next(), Some('d'));
    }

    #[test]
    fn remove() {
        let test = "Load of the Onion Ring";
        let mut v = Vec::new();
        for c in test.chars() {
            v.push(c);
        }
        let mut got = String::new();
        for _ in 0..5 {
            let result = v.remove(12);
            got.push(result);
        }
        assert_eq!(got.as_str(), "Onion");
    }

    #[test]
    fn insert() {
        let test = "Load of the Ring";
        let mut v = Vec::new();
        for c in test.chars() {
            v.push(c);
        }
        assert_eq!(v.len(), test.len());
        for (i, c) in "Onion".chars().enumerate() {
            v.insert(12 + i, c);
        }
        assert_eq!(v.len(), test.len() + "Onion".len());
        assert_eq!(v.last(), Some(&'g'));
    }

    #[test]
    fn iter_mut() {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.iter_mut().for_each(|v| *v *= 2);
        assert_eq!(v.pop(), Some(4));
        assert_eq!(v.pop(), Some(2));
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
        assert_eq!(v.buf.cap, 2);
        v.push(1);
        assert_eq!(v.len, 3);
        assert_eq!(v.buf.cap, 4);
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
        assert_eq!(v.buf.cap, 2);
    }

    #[test]
    fn buf_grow() {
        let mut buf = Buf::<u32>::default();
        buf.grow();
        assert_eq!(buf.cap, 1);
        buf.grow();
        assert_eq!(buf.cap, 2);
        buf.grow();
        assert_eq!(buf.cap, 4);
    }

    #[test]
    fn buf_default() {
        let buf = Buf::<u64>::default();
        assert_eq!(buf.cap, 0);
    }
}
