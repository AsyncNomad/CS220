//! Singly linked list.
//!
//! Consult <https://doc.rust-lang.org/book/ch15-01-box.html>.

use std::fmt::Debug;

/// Node of the list.
#[derive(Debug)]
pub struct Node<T: Debug> {
    /// Value of current node.
    pub value: T,

    /// Pointer to the next node. If it is `None`, there is no next node.
    pub next: Option<Box<Node<T>>>,
}

impl<T: Debug> Node<T> {
    /// Creates a new node.
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

/// A singly-linked list.
#[derive(Debug)]
pub struct SinglyLinkedList<T: Debug> {
    /// Head node of the list. If it is `None`, the list is empty.
    head: Option<Node<T>>,
}

impl<T: Debug> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> SinglyLinkedList<T> {
    /// Creates a new list.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Adds the given node to the front of the list.
    pub fn push_front(&mut self, value: T) {
        let mut newhead = Node::new(value);
        match self.head {
            None => self.head = Some(newhead),
            Some(_) => {
                let currenthead = self.head.take().unwrap();
                newhead.next = Some(Box::new(currenthead));
                self.head = Some(newhead);
            }
        }
    }

    /// Adds the given node to the back of the list.
    pub fn push_back(&mut self, value: T) {
        fn length<U: Debug>(list: &SinglyLinkedList<U>) -> usize {
            let mut n = 0;
            let mut currentnode = list.head.as_ref();
            while let Some(node) = currentnode {
                n += 1;
                currentnode = node.next.as_deref();
            }
            n
        }
        fn currenttail<V: Debug>(list: &mut SinglyLinkedList<V>) -> Option<&mut Node<V>> {
            let n = length(list) - 1;
            let mut nth_node = list.head.as_mut();
            for _ in 0..n {
                nth_node = match nth_node {
                    None => return None,
                    Some(node) => node.next.as_deref_mut(),
                }
            }
            nth_node
        }

        if length(self) == 0 {
            let newtail = Node::new(value);
            self.head = Some(newtail);
        } else {
            let mut newtail = Some(Box::new(Node::new(value)));
            let mut currenttail = currenttail(self);
            match currenttail {
                None => {}
                Some(node) => node.next = newtail,
            }
        }
    }

    /// Removes and returns the node at the front of the list.
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head {
            None => None,
            Some(_) => {
                let currenthead = self.head.take().unwrap();
                let currentheadvalue = currenthead.value;
                let newhead = currenthead.next.map(|x| *x);
                self.head = newhead;
                Some(currentheadvalue)
            }
        }
    }

    /// Removes and returns the node at the back of the list.
    pub fn pop_back(&mut self) -> Option<T> {
        fn length<U: Debug>(list: &SinglyLinkedList<U>) -> usize {
            let mut n = 0;
            let mut currentnode = list.head.as_ref();
            while let Some(node) = currentnode {
                n += 1;
                currentnode = node.next.as_deref();
            }
            n
        }
        fn newtail<V: Debug>(list: &mut SinglyLinkedList<V>) -> Option<&mut Node<V>> {
            let n = length(list) - 2;
            let mut newtail = list.head.as_mut();
            for _ in 0..n {
                newtail = match newtail {
                    None => return None,
                    Some(node) => node.next.as_deref_mut(),
                }
            }
            newtail
        }
        let n = length(self);
        if n == 0 {
            None
        } else if n == 1 {
            let result = self.head.take().unwrap().value;
            Some(result)
        } else {
            let mut newtail = newtail(self).unwrap();
            let currenttail = newtail.next.take();
            Some(currenttail.unwrap().value)
        }
    }
}
