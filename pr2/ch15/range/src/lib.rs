//! Iterator exercise
use num::One;
use std::fmt::Debug;
use std::iter::Iterator;
use std::ops::Add;

#[derive(Debug)]
pub struct Range<T>
where 
    T: Copy + Debug + Eq + One + Add<Output = T>,
{
    pub start: T,
    pub end: T,
}

impl<T> Iterator for Range<T>
where
    T: Copy + Debug + Eq + One + Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            return None;
        }
        let result = self.start;
        self.start = self.start + T::one();
        Some(result)
    }
}
