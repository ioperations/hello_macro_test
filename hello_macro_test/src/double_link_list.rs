#[allow(unused)]
pub mod double_link_list11 {

    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Link<T> = Option<Rc<RefCell<Node<T>>>>;

    #[allow(unused)]
    #[derive(Debug)]
    pub struct Node<T> {
        pub elem: T,
        pub next: Link<T>,
        pub prev: Link<T>,
    }

    #[allow(unused)]
    #[derive(Debug)]
    pub struct List<T> {
        pub head: Link<T>,
        pub tail: Link<T>,
    }

    #[allow(unused)]
    impl<T> Node<T> {
        fn new(ele: T) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Node {
                elem: ele,
                prev: None,
                next: None,
            }))
        }
    }

    #[allow(unused)]
    impl<T> List<T> {
        pub fn new() -> Self {
            List {
                head: None,
                tail: None,
            }
        }

        pub fn push_front(&mut self, elem: T) {
            let new_head = Node::new(elem);

            match self.head.take() {
                Some(old_value) => {
                    old_value.try_borrow_mut().unwrap().prev = Some(new_head.clone());
                    new_head.try_borrow_mut().unwrap().next = Some(old_value);
                    self.head = Some(new_head);
                }
                None => {
                    self.borrow_mut().tail = Some(new_head.clone());
                    self.borrow_mut().head = Some(new_head);
                }
            }
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|old_head| {
                match old_head.try_borrow_mut().unwrap().next.take() {
                    Some(new_head) => {
                        new_head.try_borrow_mut().unwrap().prev.take();
                        self.head = Some(new_head);
                    }
                    None => {
                        self.tail.take();
                    }
                }
                Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
            })
        }
    }

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
