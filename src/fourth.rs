use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non-empty list, need to connet the old_head
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensuring itś -2
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // not emptying list
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    // emptying list
                    self.tail.take();
                    // total: -2 old, (no new)
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            // this line's code doesn't work... node.borrow()
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn push_back(&mut self, elem: T) {
        // based on push_front
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                // empty list, need to set the tail
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // based on pop_front
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            // this line's code doesn't work... node.borrow()
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node| {
            // this line's code doesn't work... node.borrow()
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node| {
            // this line's code doesn't work... node.borrow()
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
    }
}

// Trying to implement Iter but failed.

// pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

// impl<T> List<T> {
//     pub fn iter(&self) -> Iter<T> {
//         Iter(self.head.as_ref().map(|head| head.borrow()))
//     }
// }

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = Ref<'a, T>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.take().map(|node_ref| {
//             let (next, elem) = Ref::map_split(node_ref, |node| {
//                 (&node.next, &node.elem)
//             });
//             self.0 = if next.is_some() {
//                 Some(Ref::map(next, |next| &**next.as_ref().unwrap()))
//             } else {
//                 None
//             };
//             elem
//         })
//     }
// }

#[cfg(test)]
mod test {
    // add code here
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more to make sure nothing´s corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back ----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(4);
        list.push_back(8);
        list.push_back(12);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(12));
        assert_eq!(list.pop_back(), Some(8));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(10);
        list.push_front(20);
        list.push_front(30);

        assert_eq!(&*list.peek_front().unwrap(), &30);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 30);
        assert_eq!(&*list.peek_back().unwrap(), &10);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 10);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
