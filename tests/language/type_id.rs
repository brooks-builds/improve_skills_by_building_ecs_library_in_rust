use std::{
    any::{Any, TypeId},
    collections::HashMap,
    vec,
};

#[test]
fn type_id() {
    let mut components: HashMap<TypeId, Vec<Box<dyn Any + 'static>>> = HashMap::new();
    let health = 100_u32;
    let health_type_id = TypeId::of::<u32>();
    components.insert(health_type_id, vec![Box::new(health)]);
    let speed = Speed(150);
    let speed_type_id = speed.type_id();
    components.insert(speed_type_id, vec![Box::new(speed)]);

    for (_component_type_id, component_value) in components {
        let type_id = component_value[0].type_id();
        dbg!(type_id);
    }
}

struct Speed(u32);
