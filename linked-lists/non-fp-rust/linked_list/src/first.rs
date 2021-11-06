// following along with
// https://rust-unofficial.github.io/too-many-lists/first.html
use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    None,
    Some(Box<Node>),
}

struct Node {
    element: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::None }
    }

    pub fn push(&mut self, value: i32) {
        let popped_link = mem::replace(&mut self.head, Link::None);

        let new_node = Box::new(Node {
            element: value,
            next: popped_link,
        });

        self.head = Link::Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        let popped_link = mem::replace(&mut self.head, Link::None);

        let popped_value = match popped_link {
            Link::None => Option::None,
            Link::Some(boxed_node) => {
                let value = boxed_node.element;
                self.head = boxed_node.next;
                Option::Some(value)
            }
        };

        popped_value
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
}
