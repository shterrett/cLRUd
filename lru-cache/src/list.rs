// Doubly Linked List implemented from the tutorial Learning Rust With Entirely Too Many Linked
// Lists
// http://cglab.ca/~abeinges/blah/too-many-lists/book/fourth-layout.html

use std::rc::Rc;
use std::cell::{Ref, RefCell};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None
        }))
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None
        }
    }

    pub fn peek_head(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node|
            Ref::map(node.borrow(), |node| &node.elem)
        )
    }

    pub fn peek_tail(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node|
            Ref::map(node.borrow(), |node| &node.elem)
        )
    }

    pub fn unshift(&mut self, elem: T) {
        let new_head = Node::new(elem);
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

    pub fn shift(&mut self) -> Option<T> {
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
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Node::new(elem);
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

    pub fn pop(&mut self) -> Option<T> {
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
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.shift()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop()
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn shift_and_unshift() {
        let mut list = List::new();

        assert_eq!(list.shift(), None);

        list.unshift(1);
        list.unshift(2);
        list.unshift(3);

        assert_eq!(list.shift(), Some(3));

        list.unshift(5);

        assert_eq!(list.shift(), Some(5));
        assert_eq!(list.shift(), Some(2));
        assert_eq!(list.shift(), Some(1));

        assert_eq!(list.shift(), None);
    }

    #[test]
    fn push_and_pop() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();

        list.unshift(1);
        list.unshift(2);

        assert_eq!(&*list.peek_head().unwrap(), &2);
        assert_eq!(&*list.peek_tail().unwrap(), &1);

        list.shift();

        assert_eq!(&*list.peek_head().unwrap(), &1);
        assert_eq!(&*list.peek_tail().unwrap(), &1);

        list.shift();

        assert!(list.peek_head().is_none());
        assert!(list.peek_tail().is_none());
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(4));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);

        let mut list_2 = List::new();

        list_2.push(1);
        list_2.push(2);
        list_2.push(3);

        let mut iter_rev = list_2.into_iter().rev();
        assert_eq!(iter_rev.next(), Some(3));
        assert_eq!(iter_rev.next(), Some(2));
        assert_eq!(iter_rev.next(), Some(1));
        assert_eq!(iter_rev.next(), None);
    }
}
