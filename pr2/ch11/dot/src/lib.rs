//! [num] create and [Num] trait example
//!
//! [num]: https://lib.rs/crates/num
//! [Num]: https://docs.rs/num/latest/num/traits/trait.Num.html
use num_traits::Num;

pub fn dot<N: Num + Copy>(v1: &[N], v2: &[N]) -> N {
    let mut total = N::zero();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[cfg(test)]
mod tests {
    use super::dot;

    #[test]
    fn test_dot() {
        let v1: Vec<u64> = vec![1, 2, 3];
        let v2 = vec![2, 3, 4];
        assert_eq!(dot(&v1, &v2), 20);
    }
}
