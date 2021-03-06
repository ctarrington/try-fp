// following along with
// https://rust-unofficial.github.io/too-many-lists/second.html

// ------------------- Core data structure and List API ----------------------------------

// something small to expose publicly
pub struct List<T> {
    head: Link<T>,
}

// something to hold the variation
type Link<T> = Option<Box<Node<T>>>;

// the crux of the thing
struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let boxed_node = Box::new(Node {
            element: value,
            next: self.head.take(),
        });

        self.head = Some(boxed_node);
    }

    pub fn peek_for_nosy_people(&self) -> Option<&T> {
        // goal is to avoid the move out of the Box so we need the as_ref
        // look what map and the magic dot in node.element did for us!
        // Levels of indirection just vanish!
        // https://doc.rust-lang.org/std/option/
        let boxed_node_option: Option<&Box<Node<T>>> = self.head.as_ref();
        boxed_node_option.map(|boxed_node: &Box<Node<T>>| {
            let ref_to_boxed_node: &Box<Node<T>> = boxed_node;
            let ref_to_element: &T = &ref_to_boxed_node.element;
            ref_to_element
        })
    }

    // idiomatic and dense but same as the nosy version
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_node| &boxed_node.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.element)
    }

    pub fn pop(&mut self) -> Option<T> {
        let popped_link = self.head.take();

        popped_link.map(|boxed_node| {
            self.head = boxed_node.next;
            boxed_node.element
        })
    }
}

// ------------------- iteration over Ts ----------------------------------

// into_iter provides an iterator over Ts after the list is moved into the ListIntoIter
impl<T> List<T> {
    // creates an iterator. The list is moved into the ListIntoIter and is no longer available
    pub fn into_iter(self) -> ListIntoIter<T> {
        ListIntoIter(self)
    }
}

// wrap the list so we have a place to put the iteration logic
// no need for a lifetime since the List is moved into it
pub struct ListIntoIter<T>(List<T>);

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// ------------------- iteration over references to Ts ----------------------------------

// iter provides an iterator over references to Ts after the ListIter is given a ref to the List
impl<T> List<T> {
    // creates an iterator. The list is not consumed. Iterator provides references to the elements
    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            next: self.head.as_deref().map(|node| &*node),
        }
    }
}

// we need a lifetime since a reference is used in the creation of the struct
pub struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // deref and then unbox
            self.next = node.next.as_deref().map(|node| &*node);
            &node.element
        })
    }
}

// ------------------- iteration over mutable references to Ts ----------------------------------

// create an iterator of mutable references to T given a mutable reference to the List
// note the use of the anonymous lifetime
// https://yegeun542.github.io/rust-edition-guide-ko/rust-2018/ownership-and-lifetimes/the-anonymous-lifetime.html
impl<T> List<T> {
    pub fn iter_mut(&mut self) -> ListIterMut<'_, T> {
        ListIterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct ListIterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            // deref and then unbox
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

// ------------------- Drop  ----------------------------------

// unpack and drop without copying
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut popped_link = self.head.take();
        while let Some(mut boxed_node) = popped_link {
            popped_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn simple() {
        let mut list = List::new(); // head -> None
        assert_eq!(list.pop().is_none(), true);
        assert_eq!(list.peek().is_none(), true);
        assert_eq!(list.peek_for_nosy_people().is_none(), true);
        list.push(1); // head -> Some(1, None)
        list.push(2); // head -> Some(2, Some(1, None))
        list.push(3);

        assert_eq!(list.peek().unwrap(), &3);
        assert_eq!(list.peek_for_nosy_people().unwrap(), &3);
        assert_eq!(list.pop().unwrap(), 3);
        assert_eq!(list.pop().unwrap(), 2);
        list.push(4);
        list.push(5);
        assert_eq!(list.pop().unwrap(), 5);
        assert_eq!(list.pop().unwrap(), 4);
        assert_eq!(list.pop().unwrap(), 1);
    }

    #[test]
    fn mutable_peek() {
        let mut list = List::new(); // head -> None
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(Some(&3), list.peek());
        list.peek_mut().map(|mutable_value| {
            *mutable_value = 33;
        });

        assert_eq!(Some(&33), list.peek());
        assert_eq!(Some(33), list.pop());
    }

    #[test]
    fn another_type() {
        let mut list = List::new();
        list.push("hi");
        list.push("there");
        assert_eq!("there", list.pop().unwrap());
        assert_eq!("hi", list.pop().unwrap());
    }

    #[test]
    fn iteration_into_iter() {
        let mut list = List::new();
        list.push("hi");
        list.push("there");

        let mut iterator = list.into_iter();
        assert_eq!(Some("there"), iterator.next());
        assert_eq!(Some("hi"), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn iteration_iter() {
        let mut list = List::new();
        list.push("hi");
        list.push("there");

        let mut iterator = list.iter();
        assert_eq!(Some(&"there"), iterator.next());
        assert_eq!(Some(&"hi"), iterator.next());
        assert_eq!(None, iterator.next());

        assert_eq!(Some("there"), list.pop());
    }

    #[test]
    fn iteration_iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iterator = list.iter_mut();
        assert_eq!(Some(&mut 3), iterator.next());
        assert_eq!(Some(&mut 2), iterator.next());
        assert_eq!(Some(&mut 1), iterator.next());
    }

    // TODO: need a way to test that drop does not copy
}
