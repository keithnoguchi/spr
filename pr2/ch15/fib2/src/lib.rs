//! Fibonacci
use std::iter;

pub fn fib() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    iter::from_fn(move || {
        let result = state.0;
        state = (state.1, state.0 + state.1);
        Some(result)
    })
}

#[cfg(test)]
mod tests {
    use super::fib;

    #[test]
    fn test_fib_next() {
        let mut iter = fib();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.next(), Some(21));
    }

    #[test]
    fn test_fib_take_10() {
        let result = fib().take(10).collect::<Vec<_>>();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}
