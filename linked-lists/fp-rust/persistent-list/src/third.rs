// following along with
// https://rust-unofficial.github.io/too-many-lists/third.html

use std::rc::Rc;
// ------------------- Core data structure and List API ----------------------------------

// something small to expose publicly
pub struct PersistentList<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    next: Link<T>,
    element: T,
}

impl<T> PersistentList<T> {
    pub fn new() -> Self {
        PersistentList { head: None }
    }

    // returns a List with the new value prepended to the front of the old List
    // the old List still exists
    pub fn prepend(&self, value: T) -> PersistentList<T> {
        PersistentList {
            head: Some(Rc::new(Node {
                element: value,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> PersistentList<T> {
        PersistentList {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> PersistentList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

#[cfg(test)]
mod test {
    use super::PersistentList;

    #[test]
    fn simple() {
        let empty_list = PersistentList::new();
        assert_eq!(empty_list.head(), None);

        let list_1 = empty_list.prepend(1);
        assert_eq!(empty_list.head(), None);
        assert_eq!(list_1.head(), Some(&1));

        let list_321 = list_1.prepend(2).prepend(3);
        let list_21 = list_321.tail();
        assert_eq!(list_321.head(), Some(&3));
        assert_eq!(list_21.head(), Some(&2));
    }

    #[test]
    fn iteration() {
        let list = PersistentList::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
