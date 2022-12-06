//! Tree: Non-Balancing Binary Tree
//!
//! https://doc.rust-lang.org/nomicon/borrow-splitting.html
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::ops::Deref;

// Node link, e.g. nullable pointer, pattern, similar to the list.
type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Tree<T: Ord> {
    root: Link<T>,
}

pub struct Iter<'a, T: 'a>(VecDeque<NodeIter<'a, T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.front_mut().and_then(|iter| iter.next()) {
                Some(State::Data(data)) => return Some(data),
                Some(State::Node(node)) => self.0.push_front(node.iter()),
                None => {
                    self.0.pop_front()?;
                }
            }
        }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.back_mut().and_then(|iter| iter.next_back()) {
                Some(State::Data(data)) => return Some(data),
                Some(State::Node(node)) => self.0.push_back(node.iter()),
                None => {
                    self.0.pop_back()?;
                }
            }
        }
    }
}

impl<T: Ord> Default for Tree<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> Iter<T> {
        let mut deque = VecDeque::new();
        if let Some(root) = self.root.as_ref() {
            deque.push_front(root.iter());
        }
        Iter(deque)
    }

    // no balancing whatsoever
    pub fn insert(&mut self, data: T) -> bool {
        let mut node = &mut self.root;
        loop {
            match node {
                Some(inner) => match data.cmp(&inner.data) {
                    Ordering::Less => node = &mut inner.left,
                    Ordering::Greater => node = &mut inner.right,
                    Ordering::Equal => return false,
                },
                None => {
                    *node = Some(Box::new(Node::new(data)));
                    return true;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    data: T,
}

struct NodeIter<'a, T: 'a> {
    left: Option<&'a Node<T>>,
    right: Option<&'a Node<T>>,
    data: Option<&'a T>,
}

enum State<'a, T: 'a> {
    Data(&'a T),
    Node(&'a Node<T>),
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            left: None,
            right: None,
            data,
        }
    }

    fn iter(&self) -> NodeIter<T> {
        NodeIter {
            left: self.left.as_deref(),
            right: self.right.as_deref(),
            data: Some(&self.data),
        }
    }
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = State<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        // iterate through the three possible options,
        // left node, data, or rigth node.
        match self.left.take() {
            Some(node) => Some(State::Node(node)),
            None => match self.data.take() {
                Some(data) => Some(State::Data(data)),
                None => self.right.take().map(|node| State::Node(node)),
            },
        }
    }
}

impl<'a, T> DoubleEndedIterator for NodeIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.right.take() {
            Some(node) => Some(State::Node(node)),
            None => match self.data.take() {
                Some(data) => Some(State::Data(data)),
                None => self.left.take().map(|node| State::Node(node)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, Tree};

    #[test]
    fn tree_next_back() {
        let mut tree = Tree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        let mut iter = tree.iter();
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next_back(), Some(&2));
        assert_eq!(iter.next_back(), Some(&1));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn tree_next() {
        let mut tree = Tree::new();
        tree.insert(110);
        tree.insert(2);
        tree.insert(1000);
        let mut iter = tree.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&110));
        assert_eq!(iter.next(), Some(&1000));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn tree_insert() {
        let mut tree = Tree::new();
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.insert(1), false);
    }

    #[test]
    fn node_deref() {
        let node = Node::new(9);
        assert_eq!(*node, 9);
    }

    #[test]
    fn node_new() {
        let node = Node::new(9);
        assert_eq!(node.left, None);
        assert_eq!(node.right, None);
        assert_eq!(node.data, 9);
    }

    #[test]
    fn tree_new() {
        let tree = Tree::<u32>::new();
        assert_eq!(tree.root, None);
    }
}
