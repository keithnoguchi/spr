//! fib: [Fibonacci Number]
//!
//! Presented in Chapter 11: Traits and Generics of Programming Rust
//! 2nd edition.
//!
//! [fibonacci number]: https://en.wikipedia.org/wiki/Fibonacci_number
use std::ops::Add;

pub trait Float {
    const ZERO: Self;
    const ONE: Self;
}

pub fn fib<T: Float + Add<Output = T>>(n: usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        n => fib::<T>(n - 1) + fib::<T>(n - 2),
    }
}

impl Float for f32 {
    const ZERO: f32 = 0.0;
    const ONE: f32 = 1.0;
}

impl Float for f64 {
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn fib_f32() {
        assert_eq!(0.0, super::fib::<f32>(0));
        assert_eq!(1.0, super::fib::<f32>(1));
        assert_eq!(1.0, super::fib::<f32>(2));
        assert_eq!(2.0, super::fib::<f32>(3));
        assert_eq!(3.0, super::fib::<f32>(4));
        assert_eq!(5.0, super::fib::<f32>(5));
        assert_eq!(8.0, super::fib::<f32>(6));
        assert_eq!(13.0, super::fib::<f32>(7));
        assert_eq!(21.0, super::fib::<f32>(8));
        assert_eq!(34.0, super::fib::<f32>(9));
        assert_eq!(55.0, super::fib::<f32>(10));
        assert_eq!(89.0, super::fib::<f32>(11));
        assert_eq!(144.0, super::fib::<f32>(12));
        assert_eq!(233.0, super::fib::<f32>(13));
        assert_eq!(377.0, super::fib::<f32>(14));
        assert_eq!(610.0, super::fib::<f32>(15));
        assert_eq!(987.0, super::fib::<f32>(16));
        assert_eq!(1597.0, super::fib::<f32>(17));
        assert_eq!(2584.0, super::fib::<f32>(18));
        assert_eq!(4181.0, super::fib::<f32>(19));
        assert_eq!(6765.0, super::fib::<f32>(20));
    }
}
