pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    // add code here
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            //next: mem::replace(self, List::new()),
            // was not able to compile the code below -> Definition of struct Node was mistaken.
            next: self.head.take(), 
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<& T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here
            // but its Node's next field has been set to Link::Empty
        }
    }
}

// Tuple structs are an alternative form of struct,
// useful for trivial wrappers around other types.
pub struct IntoIter<T>(List<T>);

impl <T> List<T> {
    // add code here
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl <T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iter is generic over *some* lifetime, it doesn't care
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// No lifetime here becuase List doesn't have any associated lifetimes
impl<T> List<T> {
    // We declare a fresh lifetime here for the *exact* borrow that
    // creates the iter. Now &self needs to be valid as long as the
    // Iter is around.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
       Iter { next: self.head.as_ref().map(|node| &**node) }
   } 
}

// We *do* have a lifetime here, because Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    // Need it here too, this is a type declaration
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
       IterMut { next: self.head.as_mut().map(|node| &mut **node) }
   } 
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        
        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more values
        list.push(4);
        list.push(5);
        
        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(3); list.push(2); list.push(1);

        // assert_eq!(list.peek(), 1);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_mut(), Some(&mut 1));
        list.peek_mut().map(|val| {
            *val = 8;
        });

        assert_eq!(list.peek(), Some(&8));
        assert_eq!(list.pop(), Some(8));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(3); list.push(9);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(3);
        list.push(1); list.push(10); list.push(100);
        list.push(2); list.push(20); list.push(200);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&200));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&2));
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        let a = iter.next();
        assert_eq!(x, &100);
        assert_eq!(y, &10);
        assert_eq!(z, &1);
        assert_eq!(a, Some(&3));
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(3); list.push(1); list.push(4);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 3));
    }
}