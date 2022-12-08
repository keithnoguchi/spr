//! Operator Overloading Exercise with Add trait
use std::ops::Add;

#[derive(Debug, Eq, PartialEq)]
pub struct Complex<T> {
    re: T,
    im: T,
}

// a + b = c
impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

// a + &b = c
impl<T> Add<&Self> for Complex<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

// &a + &b = c
impl<T> Add for &Complex<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Complex;

    #[test]
    fn plus() {
        let a = Complex { re: 1.9, im: 5.2 };
        let b = Complex { re: 2.1, im: 9.8 };
        assert_eq!(a + b, Complex { re: 4.0, im: 15.0 });
    }

    #[test]
    fn add() {
        use std::ops::Add;

        let a = Complex { re: 1.9, im: 5.2 };
        let b = Complex { re: 2.1, im: 9.8 };
        assert_eq!(a.add(b), Complex { re: 4.0, im: 15.0 });
    }

    #[test]
    fn plus_ref() {
        let a = Complex { re: 1.9, im: 5.2 };
        let b = Complex { re: 2.1, im: 9.8 };
        assert_eq!(a + &b, Complex { re: 4.0, im: 15.0 });
    }

    #[test]
    fn add_ref() {
        use std::ops::Add;

        let a = Complex { re: 1.9, im: 5.2 };
        let b = Complex { re: 2.1, im: 9.8 };
        assert_eq!(a.add(&b), Complex { re: 4.0, im: 15.0 });
    }

    #[test]
    fn ref_plus_ref() {
        let a = Complex { re: 1.9, im: 5.2 };
        let b = Complex { re: 2.1, im: 9.8 };
        assert_eq!(&a + &b, Complex { re: 4.0, im: 15.0 });
    }

    #[test]
    fn ref_add_ref() {
        use std::ops::Add;

        let a = Complex { re: 1.9, im: 5.2 };
        let b = Complex { re: 2.1, im: 9.8 };
        assert_eq!(a.add(&b), Complex { re: 4.0, im: 15.0 });
    }
}
