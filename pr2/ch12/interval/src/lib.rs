//! PartialOrd Example
use std::cmp::Ordering;

/// Half open interval.
#[derive(Debug, Eq, PartialEq)]
pub struct Interval<T> {
    start: T,
    end: T,
}

impl<T: PartialOrd> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.end <= other.start {
            Some(Ordering::Less)
        } else if self.start >= other.end {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Interval;
    use std::cmp::Ordering;

    #[test]
    fn partial_ord() {
        let a = Interval { start: 10, end: 20 };
        let b = Interval { start: 5, end: 10 };
        let c = Interval { start: 20, end: 21 };
        let d = Interval { start: 9, end: 15 };
        assert_eq!(a.partial_cmp(&a), Some(Ordering::Equal));
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Greater));
        assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));
        assert_eq!(a.partial_cmp(&d), None);
        assert!(!(a < d));
        assert!(!(a >= d));
        assert!(a != d);
    }
}
