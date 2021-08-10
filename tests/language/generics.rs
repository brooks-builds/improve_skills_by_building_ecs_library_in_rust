use std::any::{Any, TypeId};

#[test]
fn generics() {
    let type_id = get_type_id::<u32>();
    dbg!(type_id);
    let type_id = get_type_id::<i32>();
    dbg!(type_id);
}

fn get_type_id<T: Any>() -> TypeId {
    TypeId::of::<T>()
}
