//! A simple implementation of a persistent list   
//! following along with <https://rust-unofficial.github.io/too-many-lists/third.html>

#![warn(missing_docs)]

use std::rc::Rc;

// ------------------- Core data structure and List API ----------------------------------

/// A PersistentList cannot be changed - adding or removing returns a new List

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
    /// Create an empty PersistentList
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
    /// Returns a List with the new value prepended to the front of the old List
    /// The original List still exists
    /// ```
    /// use persistent_list::third::PersistentList;
    ///    let empty_list = PersistentList::new();
    ///    assert_eq!(empty_list.head(), None);
    ///    assert_eq!(empty_list.tail().head(), None);
    ///
    ///    let list_1 = empty_list.prepend(1);
    ///    assert_eq!(empty_list.head(), None);
    ///    assert_eq!(list_1.head(), Some(&1));
    ///
    ///    let list_321 = list_1.prepend(2).prepend(3);
    ///    let list_21 = list_321.tail();
    ///    assert_eq!(list_321.head(), Some(&3));
    ///    assert_eq!(list_21.head(), Some(&2));
    /// ```
    pub fn prepend(&self, value: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                element: value,
                // use Rc::clone to make it explicit that we are only bumping the reference count
                next: self.head.as_ref().map(|rc_node| Rc::clone(&rc_node)),
            })),
        }
    }

    /// Returns a List with the first item removed
    /// The original List still exists
    pub fn tail(&self) -> Self {
        Self {
            head: self
                .head
                .as_ref()
                .and_then(|node| node.next.as_ref().map(|rc_node| Rc::clone(&rc_node))),
        }
    }

    /// Returns the first value held by the list
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }
}

// ------------------- iteration over references to Ts ----------------------------------

/// struct that is returned from iter. No need for client code to create these or worry about them
/// in any way.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> PersistentList<T> {
    /// Returns an iterator over the items in the list
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
    use std::cell::RefCell;

    #[test]
    fn simple() {
        let empty_list = PersistentList::new();
        assert_eq!(empty_list.head(), None);
        assert_eq!(empty_list.tail().head(), None);

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
        let empty_list: PersistentList<i32> = PersistentList::new();
        let mut iter = empty_list.iter();
        assert_eq!(iter.next(), None);

        let list = PersistentList::new().prepend(1).prepend(2).prepend(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));

        let list_421 = list.tail().prepend(4);
        let mut iter_321 = list.iter();
        let mut iter_421 = list_421.iter();
        assert_eq!(iter_321.next(), Some(&3));
        assert_eq!(iter_321.next(), Some(&2));
        assert_eq!(iter_321.next(), Some(&1));

        assert_eq!(iter_421.next(), Some(&4));
        assert_eq!(iter_421.next(), Some(&2));
        assert_eq!(iter_421.next(), Some(&1));
    }

    #[test]
    fn drop() {
        struct Watcher {
            events: RefCell<Vec<String>>,
        }

        impl Watcher {
            fn new() -> Self {
                Self {
                    events: RefCell::new(vec![]),
                }
            }

            fn push(&self, value: String) {
                self.events.borrow_mut().push(value);
            }

            fn list(&self) -> String {
                self.events.borrow().join(",")
            }
        }

        struct Thing<'a> {
            value: i32,
            watcher: &'a Watcher,
        }

        impl<'a> Drop for Thing<'a> {
            fn drop(&mut self) {
                self.watcher.push(format!("dropping {} ", self.value));
            }
        }

        let watcher = Watcher::new();

        {
            let list_1: PersistentList<Thing> = PersistentList::default().prepend(Thing {
                value: 1,
                watcher: &watcher,
            });
            assert_eq!(list_1.head().map(|thing| thing.value), Some(1));

            {
                let list_321 = list_1
                    .prepend(Thing {
                        value: 2,
                        watcher: &watcher,
                    })
                    .prepend(Thing {
                        value: 3,
                        watcher: &watcher,
                    });
                assert_eq!(list_321.head().map(|thing| thing.value), Some(3));
                watcher.push("done with list_321".to_string());
            }

            watcher.push("still using list_1".to_string());
            assert_eq!(list_1.head().map(|thing| thing.value), Some(1));
            watcher.push("done with list_1".to_string());
        }
        assert_eq!(watcher.list(), "done with list_321,dropping 3 ,dropping 2 ,still using list_1,done with list_1,dropping 1 ".to_string());
    }
}
