//! Deref and DerefMut exercise
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

pub struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn main() {
    let mut s = Selector {
        elements: vec!["one", "two", "three"],
        current: 0,
    };

    println!("s.len()={}", s.len());
    display(&s);
    s.current = 1;
    display(&s);
    *s = "II";
    display(&s);
    display_generic(&s as &str);
}

fn display(s: &str) {
    println!("{s}");
}

fn display_generic<T: Display + ?Sized>(s: &T) {
    println!("{s}");
}
