//! RefWithFlag: Reference with Packed Bool Flag
//!
//! *const T, a shared raw pointer, example demonstrated in
//! [Programming Rust, 2nd Edition], page 642.
//!
//! [programming rust, 2nd edition]: https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::mem::align_of;

pub struct RefWithFlag<'a, T> {
    ptr_and_bit: usize,
    behaves_like: PhantomData<&'a T>,
}

impl<'a, T> Debug for RefWithFlag<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RefWithFlag")
            .field("ptr_and_bit", &self.ptr_and_bit)
            .finish()
    }
}

impl<'a, T> AsRef<T> for RefWithFlag<'a, T> {
    fn as_ref(&self) -> &T {
        unsafe {
            let ptr = (self.ptr_and_bit & !1) as *const T;
            &*ptr
        }
    }
}

impl<'a, T> From<(&'a T, bool)> for RefWithFlag<'a, T> {
    fn from(from: (&'a T, bool)) -> Self {
        assert!(align_of::<T>() % 2 == 0);
        Self {
            ptr_and_bit: from.0 as *const T as usize | from.1 as usize,
            behaves_like: PhantomData,
        }
    }
}

impl<'a, T> RefWithFlag<'a, T> {
    pub fn as_flag(&self) -> bool {
        self.ptr_and_bit & 1 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::RefWithFlag;

    #[test]
    fn from_and_as_ref() {
        let v = vec![11, 22, 33];
        let flagged = RefWithFlag::from((&v, true));
        let unflagged = RefWithFlag::from((&v, false));
        assert_eq!(flagged.as_ref()[0], 11);
        assert_eq!(unflagged.as_ref()[1], 22);
        assert_eq!(flagged.as_ref()[2], 33);
        assert_eq!(flagged.as_ref().len(), unflagged.as_ref().len());
        assert_eq!(unflagged.as_ref().len(), 3);
    }

    #[test]
    fn from_and_as_flag() {
        let v = vec![10, 20, 30];
        let flagged = RefWithFlag::from((&v, true));
        let unflagged = RefWithFlag::from((&v, false));
        assert!(flagged.as_flag());
        assert!(!unflagged.as_flag());
    }
}
