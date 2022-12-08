//! Operator Overloading Exercise with Add trait
use std::ops::{Add, AddAssign, Neg};

#[derive(Debug)]
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

// -a = c
impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

// -&a = c
impl<T> Neg for &Complex<T>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            re: -self.re,
            im: -self.im,
        }
    }
}

// a += b
impl<T> AddAssign for Complex<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
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

    #[test]
    fn minus() {
        let a = Complex { re: 1.1, im: -1.2 };

        assert_eq!(-a, Complex { re: -1.1, im: 1.2 });
    }

    #[test]
    fn neg() {
        use std::ops::Neg;

        let a = Complex { re: 1.1, im: -1.2 };

        assert_eq!(a.neg(), Complex { re: -1.1, im: 1.2 });
    }

    #[test]
    fn minus_ref() {
        let a = &Complex { re: 1.1, im: -1.2 };

        assert_eq!(-a, Complex { re: -1.1, im: 1.2 });
    }

    #[test]
    fn neg_ref() {
        use std::ops::Neg;

        let a = &Complex { re: 1.1, im: -1.2 };

        assert_eq!(a.neg(), Complex { re: -1.1, im: 1.2 });
    }

    #[test]
    fn plus_equal() {
        let mut a = Complex { re: 1.9, im: 5.2 };
        let b = Complex { re: 2.1, im: 9.8 };

        a += b;
        assert_eq!(a, Complex { re: 4.0, im: 15.0 });
    }

    #[test]
    fn add_assign() {
        use std::ops::AddAssign;

        let mut a = Complex { re: 1.9, im: 5.2 };
        let b = Complex { re: 2.1, im: 9.8 };

        a.add_assign(b);
        assert_eq!(a, Complex { re: 4.0, im: 15.0 });
    }

    #[test]
    fn equal() {
        let a = Complex { re: 1.9, im: 5.2 };
        let b = Complex { re: 1.9, im: 5.2 };

        assert!(a == b);
    }

    #[test]
    fn eq() {
        use std::cmp::PartialEq;

        let a = Complex { re: 1.9, im: 5.2 };
        let b = Complex { re: 1.9, im: 5.2 };

        assert!(a.eq(&b));
    }
}
