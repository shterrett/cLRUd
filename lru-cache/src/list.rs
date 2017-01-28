// Doubly Linked List implemented from the tutorial Learning Rust With Entirely Too Many Linked
// Lists
// http://cglab.ca/~abeinges/blah/too-many-lists/book/fourth-layout.html

use std::rc::Rc;
use std::cell::RefCell;
use std::hash::{ Hash, Hasher };

pub struct List<T>
    where T: Hash + Eq {
    head: Option<Link<T>>,
    tail: Option<Link<T>>
}

pub type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct Node<T>
    where T: Hash + Eq {
    elem: T,
    next: Option<Link<T>>,
    prev: Option<Link<T>>
}

impl<T> Node<T>
    where T: Hash + Eq {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None
        }))
    }
}

impl<T> PartialEq for Node<T>
    where T: Hash + Eq {
    fn eq(&self, other: &Node<T>) -> bool {
        self.elem == other.elem
    }
}

impl<T> Eq for Node<T> where T: Hash + Eq {}

impl<T> Hash for Node<T>
    where T: Hash + Eq {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elem.hash(state);
    }}

pub struct IntoIter<T>(List<T>) where T: Hash + Eq;

impl<T> List<T>
    where T: Hash + Eq {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None
        }
    }

    pub fn peek_head(&self) -> Option<&Link<T>> {
        self.head.as_ref()
    }

    pub fn peek_tail(&self) -> Option<&Link<T>> {
        self.tail.as_ref()
    }

    pub fn unshift(&mut self, new_head: Link<T>) {
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn shift(&mut self) -> Option<Link<T>> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(next) => {
                    next.borrow_mut().prev.take();
                    self.head = Some(next);
                }
                None => {
                    self.tail.take();
                }
            }
            old_head
        })
    }

    pub fn push(&mut self, new_tail: Link<T>) {
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
    }

    pub fn pop(&mut self) -> Option<Link<T>> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(prev) => {
                    prev.borrow_mut().next.take();
                    self.tail = Some(prev);
                }
                None => {
                    self.head.take();
                }
            }
            old_tail
        })
    }

    pub fn remove(&mut self, target: &Link<T>) {
        if target.borrow().next.is_none() {
            self.pop();
        } else if target.borrow().prev.is_none() {
            self.shift();
        } else {
            let mut node = target.borrow_mut();
            let next = node.next.take().unwrap();
            let prev = node.prev.take().unwrap();
            {
                next.borrow_mut().prev = Some(prev.clone());
            }
            {
                prev.borrow_mut().next = Some(next.clone());
            }
        }
    }

    pub fn promote(&mut self, target: Link<T>) {
        self.remove(&target);
        self.unshift(target);
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T>
    where T: Hash + Eq {
    type Item = Link<T>;
    fn next(&mut self) -> Option<Link<T>> {
        self.0.shift()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T>
    where T: Hash + Eq {
    fn next_back(&mut self) -> Option<Link<T>> {
        self.0.pop()
    }
}

#[cfg(test)]
mod test {
    use super::{ List, Node };

    #[test]
    fn shift_and_unshift() {
        let mut list = List::new();

        assert_eq!(list.shift(), None);

        list.unshift(Node::new(1));
        list.unshift(Node::new(2));
        list.unshift(Node::new(3));

        assert_eq!(list.shift(), Some(Node::new(3)));

        list.unshift(Node::new(5));

        assert_eq!(list.shift(), Some(Node::new(5)));
        assert_eq!(list.shift(), Some(Node::new(2)));
        assert_eq!(list.shift(), Some(Node::new(1)));

        assert_eq!(list.shift(), None);
    }

    #[test]
    fn push_and_pop() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(Node::new(1));
        list.push(Node::new(2));
        list.push(Node::new(3));

        assert_eq!(list.pop(), Some(Node::new(3)));
        assert_eq!(list.pop(), Some(Node::new(2)));

        list.push(Node::new(4));

        assert_eq!(list.pop(), Some(Node::new(4)));
        assert_eq!(list.pop(), Some(Node::new(1)));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();

        list.unshift(Node::new(1));
        list.unshift(Node::new(2));

        assert_eq!(&*list.peek_head().unwrap(), &Node::new(2));
        assert_eq!(&*list.peek_tail().unwrap(), &Node::new(1));

        list.shift();

        assert_eq!(&*list.peek_head().unwrap(), &Node::new(1));
        assert_eq!(&*list.peek_tail().unwrap(), &Node::new(1));

        list.shift();

        assert!(list.peek_head().is_none());
        assert!(list.peek_tail().is_none());
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(Node::new(1));
        list.push(Node::new(2));
        list.push(Node::new(3));
        list.push(Node::new(4));

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(Node::new(1)));
        assert_eq!(iter.next_back(), Some(Node::new(4)));
        assert_eq!(iter.next(), Some(Node::new(2)));
        assert_eq!(iter.next_back(), Some(Node::new(3)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);

        let mut list_2 = List::new();

        list_2.push(Node::new(1));
        list_2.push(Node::new(2));
        list_2.push(Node::new(3));

        let mut iter_rev = list_2.into_iter().rev();
        assert_eq!(iter_rev.next(), Some(Node::new(3)));
        assert_eq!(iter_rev.next(), Some(Node::new(2)));
        assert_eq!(iter_rev.next(), Some(Node::new(1)));
        assert_eq!(iter_rev.next(), None);
    }

    #[test]
    fn remove() {
        let mut list = List::new();
        list.push(Node::new(1));
        list.push(Node::new(2));

        let node = Node::new(3);
        let node_ref = node.clone();

        list.push(node);
        list.push(Node::new(4));

        list.remove(&node_ref);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(Node::new(1)));
        assert_eq!(iter.next(), Some(Node::new(2)));
        assert_eq!(iter.next(), Some(Node::new(4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn remove_head() {
        let mut list = List::new();
        let node = Node::new(1);
        let node_ref = node.clone();

        list.push(node);
        list.push(Node::new(2));
        list.push(Node::new(3));
        list.push(Node::new(4));

        list.remove(&node_ref);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(Node::new(2)));
        assert_eq!(iter.next(), Some(Node::new(3)));
        assert_eq!(iter.next(), Some(Node::new(4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn remove_tail() {
        let mut list = List::new();
        list.push(Node::new(1));
        list.push(Node::new(2));
        list.push(Node::new(3));

        let node = Node::new(4);
        let node_ref = node.clone();

        list.push(node);

        list.remove(&node_ref);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(Node::new(1)));
        assert_eq!(iter.next(), Some(Node::new(2)));
        assert_eq!(iter.next(), Some(Node::new(3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn promote() {
        let mut list = List::new();
        list.push(Node::new(1));
        list.push(Node::new(2));

        let node = Node::new(3);
        let node_ref = node.clone();

        list.push(node);
        list.push(Node::new(4));

        list.promote(node_ref);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(Node::new(3)));
        assert_eq!(iter.next(), Some(Node::new(1)));
        assert_eq!(iter.next(), Some(Node::new(2)));
        assert_eq!(iter.next(), Some(Node::new(4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn promote_head() {
        let mut list = List::new();
        let node = Node::new(1);
        let node_ref = node.clone();

        list.push(node);
        list.push(Node::new(2));
        list.push(Node::new(3));
        list.push(Node::new(4));

        list.promote(node_ref);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(Node::new(1)));
        assert_eq!(iter.next(), Some(Node::new(2)));
        assert_eq!(iter.next(), Some(Node::new(3)));
        assert_eq!(iter.next(), Some(Node::new(4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn promote_tail() {
        let mut list = List::new();
        list.push(Node::new(1));
        list.push(Node::new(2));
        list.push(Node::new(3));

        let node = Node::new(4);
        let node_ref = node.clone();

        list.push(node);

        list.promote(node_ref);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(Node::new(4)));
        assert_eq!(iter.next(), Some(Node::new(1)));
        assert_eq!(iter.next(), Some(Node::new(2)));
        assert_eq!(iter.next(), Some(Node::new(3)));
        assert_eq!(iter.next(), None);
    }
}
