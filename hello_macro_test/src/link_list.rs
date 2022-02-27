use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[allow(unused)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    #[allow(unused)]
    pub fn new() -> Self {
        List { head: None }
    }

    #[allow(unused)]
    pub fn push(&mut self, ele: T) {
        let new_node = Box::new(Node {
            elem: ele,
            next: mem::replace(&mut self.head, None),
        });
        self.head = Some(new_node);
    }

    #[allow(unused)]
    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}
