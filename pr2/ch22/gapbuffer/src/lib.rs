//! GapBuffer
//!
//! Example demonstrated in [Programming Rust] 2nd Edition, page 651.
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::ops::Range;
use std::ptr::{copy, copy_nonoverlapping, drop_in_place, write};
use tracing::{debug, instrument, trace};

pub struct GapBuffer<T> {
    buf: Vec<T>,
    gap: Range<usize>,
}

impl<T> Drop for GapBuffer<T> {
    #[instrument(name = "GapBuffer::drop")]
    fn drop(&mut self) {
        unsafe {
            // before the gap
            for i in 0..self.gap.start {
                drop_in_place(self.space_mut(i))
            }
            // after the gap
            for i in self.gap.end..self.capacity() {
                drop_in_place(self.space_mut(i))
            }
        }
        debug!("dropped");
    }
}

impl<T> Debug for GapBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GapBuffer")
            .field("buf.len", &self.len())
            .field("buf.capacity", &self.capacity())
            .field("gap", &self.gap)
            .field("gap.len", &self.gap.len())
            .finish()
    }
}

impl<T> Default for GapBuffer<T> {
    fn default() -> Self {
        Self {
            buf: vec![],
            gap: Range::default(),
        }
    }
}

impl<T> GapBuffer<T> {
    #[instrument(name = "GapBuffer::new")]
    pub fn new() -> Self {
        Self::default()
    }

    #[instrument(name = "GapBuffer::capacity")]
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    #[instrument(name = "GapBuffer::len")]
    pub fn len(&self) -> usize {
        self.capacity() - self.gap.len()
    }

    #[instrument(name = "GapBuffer::is_empty")]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[instrument(name = "GapBuffer::get")]
    pub fn get(&self, index: usize) -> Option<&T> {
        let raw = self.index_to_raw(index);
        if raw < self.capacity() {
            unsafe { Some(&*self.space(raw)) }
        } else {
            None
        }
    }

    #[instrument(name = "GapBuffer::insert_iter", skip(iterable))]
    pub fn insert_iter<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = T>,
    {
        for val in iterable {
            self.insert(val)
        }
    }

    #[instrument(name = "GapBuffer::insert", skip(val))]
    pub fn insert(&mut self, val: T) {
        if self.gap.is_empty() {
            self.enlarge_gap();
        }
        unsafe {
            let index = self.gap.start;
            write(self.space_mut(index), val);
        }
        self.gap.start += 1;
    }

    /// panic if `pos` is out of bounds.
    #[instrument(name = "GapBuffer::set_position")]
    pub fn set_position(&mut self, pos: usize) {
        if pos > self.len() {
            panic!("index {} out of range for GapBuffer", pos);
        }
        unsafe {
            let gap = self.gap.clone();
            match pos.cmp(&gap.start) {
                Ordering::Equal => return, // nothing to do
                Ordering::Greater => {
                    // shift n data before the gap
                    let n = pos - gap.start;
                    copy(self.space(gap.end), self.space_mut(gap.start), n);
                }
                Ordering::Less => {
                    // shift n data after the gap
                    let n = gap.start - pos;
                    copy(self.space(pos), self.space_mut(gap.end - n), n);
                }
            }
            self.gap = pos..pos + gap.len();
        }
    }

    #[instrument(name = "GapBuffer::enlarge_cap")]
    fn enlarge_gap(&mut self) {
        let mut new_capacity = self.capacity() * 2;
        if new_capacity == 0 {
            new_capacity = 4;
        }
        let after_gap_len = self.capacity() - self.gap.end;
        let new_gap = self.gap.start..new_capacity - after_gap_len;
        let mut new = Vec::with_capacity(new_capacity);

        unsafe {
            // before the gap
            let src = self.space(0);
            let dst = new.as_mut_ptr();
            copy_nonoverlapping(src, dst, self.gap.start);
            // after the gap.
            let src = self.space(self.gap.end);
            let dst = new.as_mut_ptr().add(new_gap.end);
            copy_nonoverlapping(src, dst, after_gap_len);
        }
        self.buf = new;
        self.gap = new_gap;
    }

    #[instrument(name = "GapBuffer::index_to_raw")]
    fn index_to_raw(&self, index: usize) -> usize {
        if index < self.gap.start {
            index
        } else {
            index + self.gap.end
        }
    }

    #[instrument(name = "GapBuffer::space")]
    unsafe fn space(&self, index: usize) -> *const T {
        trace!("getting *const T");
        self.buf.as_ptr().add(index)
    }

    #[instrument(name = "GapBuffer::space_mut")]
    unsafe fn space_mut(&mut self, index: usize) -> *mut T {
        trace!("getting *mut T");
        self.buf.as_mut_ptr().add(index)
    }
}

#[cfg(test)]
mod tests {
    use super::GapBuffer;

    #[test]
    fn capacity() {
        let buf = GapBuffer::<&str>::new();
        assert_eq!(buf.capacity(), 0);
    }

    #[test]
    fn len() {
        let buf = GapBuffer::<u8>::new();
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn get() {
        let buf = GapBuffer::<char>::new();
        assert_eq!(buf.get(0), None);
    }
}
