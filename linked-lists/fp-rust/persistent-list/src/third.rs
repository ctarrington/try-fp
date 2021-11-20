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
        Self { head: None }
    }
}

impl<T> Default for PersistentList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PersistentList<T> {
    // returns a List with the new value prepended to the front of the old List
    // the old List still exists
    pub fn prepend(&self, value: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                element: value,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

// ------------------- iteration over references to Ts ----------------------------------

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

// ------------------- drop ----------------------------------

impl<T> Drop for PersistentList<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
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

    #[test]
    fn drop() {
        struct Thing {
            value: i32,
        }

        impl Drop for Thing {
            fn drop(&mut self) {
                println!("dropping {} ", self.value);
            }
        }

        let list_1: PersistentList<Thing> = PersistentList::default().prepend(Thing { value: 1 });
        assert_eq!(list_1.head().map(|thing| thing.value), Some(1));

        {
            let list_321 = list_1
                .prepend(Thing { value: 2 })
                .prepend(Thing { value: 3 });
            assert_eq!(list_321.head().map(|thing| thing.value), Some(3));
            println!("done with list_321");
        }

        println!("still using list_1");
        assert_eq!(list_1.head().map(|thing| thing.value), Some(1));
        println!("done with list_1");
    }
}
