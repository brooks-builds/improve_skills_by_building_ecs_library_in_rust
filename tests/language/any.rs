use std::any::Any;

#[test]
fn any_trait() {
    let u32s = FavoriteThings {
        thing: Box::new(10_u32),
    };
    let _floats = FavoriteThings {
        thing: Box::new(50.0_f32),
    };

    let extracted_u32 = u32s.get::<u32>().unwrap();
    assert_eq!(*extracted_u32, 10);
}

struct FavoriteThings {
    thing: Box<dyn Any + 'static>,
}

impl FavoriteThings {
    pub fn get<T: Any + 'static>(&self) -> Option<&T> {
        self.thing.downcast_ref()
    }
}
