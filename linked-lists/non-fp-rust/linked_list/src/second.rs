// following along with
// https://rust-unofficial.github.io/too-many-lists/second.html

// something small to expose publicly
pub struct List<T> {
    head: Link<T>,
}

// wrap the list so we have a place to put the iteration logic
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
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
        // not idiomatic rust, but look what map and the magic dot in node.element did for us!
        // Levels of indirection just vanish!
        let boxed_node_option: Option<&Box<Node<T>>> = self.head.as_ref();
        boxed_node_option.map(|boxed_node: &Box<Node<T>>| &boxed_node.element)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|boxed_node| &boxed_node.element)
    }

    pub fn pop(&mut self) -> Option<T> {
        let popped_link = self.head.take();

        popped_link.map(|boxed_node| {
            self.head = boxed_node.next;
            boxed_node.element
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

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
    fn another_type() {
        let mut list = List::new();
        list.push("hi");
        list.push("there");
        assert_eq!("there", list.pop().unwrap());
        assert_eq!("hi", list.pop().unwrap());
    }

    #[test]
    fn iteration() {
        let mut list = List::new();
        list.push("hi");
        list.push("there");

        let mut iterator = list.into_iter();
        assert_eq!(Some("there"), iterator.next());
        assert_eq!(Some("hi"), iterator.next());
        assert_eq!(None, iterator.next());
    }
}
