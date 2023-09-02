

#[cfg(test)]
mod tests {
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;


    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_box_smart_pointers() {
        let x = Box::new(50);

        #[derive(Debug)]
        struct Node {
            id: u32,
            next: Option<Box<Node>>
        }

        let nodes = Box::new(
            Node{ id: 0, next: Some(
                Box::new(Node{ id: 1, next: Some(
                    Box::new(Node{ id: 2, next: None})
                )})
            )}
        );

        dbg!(nodes);
    }

    #[test]
    #[allow(dead_code, unused_variables)]
    fn tests_reference_counter() {

        let mut x = Rc::new(RefCell::new(50));
        let y = Rc::clone(&x);

        *x.borrow_mut() = 70;

        dbg!(x.borrow());
        dbg!(y.borrow());

        #[derive(Debug, Clone)]
        struct  House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>
        }

        #[derive(Debug, Clone)]
        struct Furniture {
            id: String,
            description: String,
            house: Weak<House>
        }

        let mut house_1 = Rc::new(House{
            address_number: 1,
            street: String::from("Main Street"),
            furniture: RefCell::new(vec!())
        });

        let table = Rc::new(Furniture{
            id: String::from("1"),
            description: String::from("Table"),
            house: Rc::downgrade(&house_1)
        });

        let desk = Rc::new(Furniture{
            id: String::from("2"),
            description: String::from("Desk"),
            house: Rc::downgrade(&house_1)
        });

        house_1.furniture.borrow_mut().push(Rc::clone(&table));
        house_1.furniture.borrow_mut().push(Rc::clone(&desk));

        dbg!(house_1);

        

    }
}