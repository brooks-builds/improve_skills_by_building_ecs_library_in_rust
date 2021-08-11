use std::{cell::RefCell, rc::Rc};

#[test]
fn interior_mutability() {
    let number = 0;
    let wrapped_number = Rc::new(RefCell::new(number));
    dbg!(wrapped_number.clone());
    add_one(wrapped_number.clone());
    dbg!(wrapped_number);
}

fn add_one(wrapped_number: Rc<RefCell<u32>>) {
    let cloned_number = {
        let other_borrowed_number = wrapped_number.borrow();
        *other_borrowed_number
    };
    let mut borrowed_number = wrapped_number.borrow_mut();
    *borrowed_number += 1;
}
