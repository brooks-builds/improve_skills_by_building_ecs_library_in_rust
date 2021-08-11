use std::any::{Any, TypeId};

#[test]
fn boxes() {
    let number = 15.0_f32;
    let _type_id = get_type_id(Box::new(number));
}

fn get_type_id(boxed_thing: Box<dyn Any>) -> TypeId {
    boxed_thing.type_id()
}
