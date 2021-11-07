// following along with
// https://rust-unofficial.github.io/too-many-lists/second.html

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

    pub fn pop(&mut self) -> Option<T> {
        let popped_link = self.head.take();

        popped_link.map(|boxed_node| {
            self.head = boxed_node.next;
            boxed_node.element
        })
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
        list.push(1); // head -> Some(1, None)
        list.push(2); // head -> Some(2, Some(1, None))
        list.push(3);
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
}
