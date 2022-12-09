//! List: Singly Linked List
//!
//! It's a good example how to model the nullable raw pointer
//! without introducing the unconditional behavior through
//! the combination of Option<T> and Box<T>.
//!
//! It's demonstrated in the [ownership] chapter of the
//! [Rustonomicon].
//!
//! [ownership]: https://doc.rust-lang.org/nomicon/borrow-splitting.html
//! [rustnomicon]: https://doc.rust-lang.org/nomicon/
use std::ops::Deref;

// This Link type alias is the key.  It represents the nullable pointer
// without introducing the undefined behavior, dereferencing null, etc.
type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
}

pub struct Iter<'a, T: 'a>(Option<&'a Node<T>>);
pub struct IterMut<'a, T: 'a>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref();
            &node.data
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut().map(|node| &mut *node);
            &mut node.data
        })
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_deref())
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_deref_mut())
    }

    pub fn push_front(&mut self, data: T) {
        let mut node = Node::new(data);
        node.next = self.head.take();
        self.head = Some(Box::new(node));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    next: Link<T>,
    data: T,
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { next: None, data }
    }
}

#[cfg(test)]
mod tests {
    use super::{List, Node};

    #[test]
    fn list_iter_mut() {
        let mut list = List::new();
        list.push_front("first".to_string());
        list.push_front("second".to_string());
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut "second".to_string()));
        assert_eq!(iter.next(), Some(&mut "first".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn list_push_and_pop() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);
        list.push_front("first");
        list.push_front("second");
        assert_eq!(list.pop_front(), Some("second"));
    }

    #[test]
    fn list_push_front() {
        let mut list = List::new();
        list.push_front("first".to_string());
        list.push_front("second".to_string());
    }

    #[test]
    fn list_new() {
        let list = List::<String>::new();
        assert_eq!(list.head, None);
    }

    #[test]
    fn node_new() {
        let node = Node::new("first".to_string());
        assert_eq!(*node, "first".to_string());
    }
}
