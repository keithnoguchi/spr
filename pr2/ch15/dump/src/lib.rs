//! IntoIterator exercise
pub fn dump<T, U>(t: T)
where
    T: IntoIterator<Item = U>,
    U: std::fmt::Debug,
{
    for u in t {
        print!("{u:?} ");
    }
}
