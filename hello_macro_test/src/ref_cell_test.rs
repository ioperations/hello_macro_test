mod ref_cell_test11 {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    #[allow(unused)]
    struct TestStruct {
        id: i32,
        name: String,
    }

    #[derive(Debug)]
    #[allow(unused)]
    struct MyStruct {
        z: Option<Rc<RefCell<TestStruct>>>,
        my_world: i32,
    }

    impl TestStruct {
        #[allow(unused)]
        fn new() -> Self {
            TestStruct {
                id: 0,
                name: "".to_string(),
            }
        }
    }

    impl MyStruct {
        #[allow(unused)]
        fn new() -> Self {
            MyStruct {
                z: Some(Rc::new(RefCell::new(TestStruct {
                    id: 0,
                    name: "".to_string(),
                }))),
                my_world: 0,
            }
        }

        #[allow(unused)]
        fn test1(&mut self) {
            if let Some(old_value) = self.z.take() {
                old_value.try_borrow_mut().unwrap().id = 100;
                self.z = Some(old_value);
            }
        }
    }

    #[test]
    fn test_my_struct() {
        let mut my_struct = MyStruct::new();

        println!("{:#?}", my_struct);
        my_struct.test1();
        println!("{:#?}", my_struct);
    }
}
