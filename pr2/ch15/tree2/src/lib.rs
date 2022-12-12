use std::cmp::{Ord, Ordering};

pub struct Tree<T: Ord>(Link<T>);

impl<T: Ord> Default for Tree<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, data: T) -> bool {
        let mut link = &mut self.0;
        while let Some(node) = link {
            link = match data.cmp(&node.data) {
                Ordering::Less => &mut node.left,
                Ordering::Greater => &mut node.right,
                Ordering::Equal => return false,
            };
        }
        *link = Some(Box::new(Node::new(data)));
        true
    }

    pub fn iter(&self) -> DepthFirstIter<T> {
        let mut iter = DepthFirstIter::default();
        iter.push_left_edge(&self.0);
        iter
    }
}

pub struct DepthFirstIter<'a, T: 'a + Ord> {
    unvisited: Vec<&'a Node<T>>,
}

impl<'a, T: 'a + Ord> Default for DepthFirstIter<'a, T> {
    fn default() -> Self {
        Self {
            unvisited: Vec::default(),
        }
    }
}

impl<'a, T: 'a + Ord> Iterator for DepthFirstIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.unvisited.pop().map(|node| {
            self.push_left_edge(&node.right);
            &node.data
        })
    }
}

impl<'a, T: 'a + Ord> DepthFirstIter<'a, T> {
    fn push_left_edge(&mut self, mut link: &'a Link<T>) {
        while let Some(node) = link {
            link = &node.left;
            self.unvisited.push(node);
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T: Ord> {
    data: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            left: None,
            right: None,
        }
    }
}
