//! IntoIterator exercise
use dump::dump;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashSet, VecDeque};

macro_rules! dump {
    ($iterable:expr, $kind:expr) => {
        dump($iterable);
        println!(": {}", $kind);
    };
}

fn main() {
    let mut iterable = Vec::from_iter(0..10);
    let kind = "vector";
    dump!(&iterable, ["&", kind].concat());
    dump!(&mut iterable, ["&mut ", kind].concat());
    dump!(iterable, kind);

    let mut iterable = VecDeque::from_iter(0..10);
    let kind = "queue";
    dump!(&iterable, ["&", kind].concat());
    dump!(&mut iterable, ["&mut ", kind].concat());
    dump!(iterable, kind);

    let iterable = BinaryHeap::from_iter(0..10);
    let kind = "max-heap";
    dump!(&iterable, ["&", kind].concat());
    //dump!(&mut iterable, ["&mut ", kind].concat()); // no mutation allowed
    dump!(iterable, kind);

    let iterable = BinaryHeap::from_iter((0..10).map(Reverse));
    let kind = "min-heap";
    dump!(&iterable, ["&", kind].concat());
    //dump!(&mut iterable, ["&mut ", kind].concat()); // no mutation allowed
    dump!(iterable, kind);

    let iterable = HashSet::<i32>::from_iter(0..10);
    let kind = "hash-set";
    dump!(&iterable, ["&", kind].concat());
    //dump!(&mut iterable, ["&mut ", kind].concat()); // no mutation allowed
    dump!(iterable, kind);

    let iterable = BTreeSet::from_iter(0..10);
    let kind = "btree-set";
    dump!(&iterable, ["&", kind].concat());
    //dump!(&mut iterable, ["&mut ", kind].concat()); // no mutation allowed
    dump!(iterable, kind);

    let iterable = BTreeSet::from_iter((0..10).map(Reverse));
    let kind = "btree-set-rev";
    dump!(&iterable, ["&", kind].concat());
    //dump!(&mut iterable, ["&mut ", kind].concat()); // no mutation allowed
    dump!(iterable, kind);
}
