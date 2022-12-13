use std::collections::HashMap;
use std::fmt::{self, Display};

#[derive(Default)]
pub struct Table(HashMap<String, Vec<String>>);

impl Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for (artist, works) in &self.0 {
            if !first {
                writeln!(f)?
            }
            writeln!(f, "{artist}")?;
            for (i, work) in works.iter().enumerate() {
                writeln!(f, "{}: {work}", i + 1)?;
            }
            first = false;
        }
        Ok(())
    }
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<K: ToString, V: ToString>(&mut self, k: K, v: V) {
        self.0
            .entry(k.to_string())
            .or_insert_with(|| Vec::with_capacity(1))
            .push(v.to_string());
    }

    pub fn sort_works(&mut self) {
        self.0.values_mut().for_each(|works| works.sort());
    }
}
