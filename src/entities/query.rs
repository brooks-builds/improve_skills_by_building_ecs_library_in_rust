use std::{
    any::{Any, TypeId},
    cell::RefCell,
    rc::Rc,
};

use eyre::Result;

use crate::custom_errors::CustomErrors;

use super::Entities;

#[derive(Debug)]
pub struct Query<'a> {
    map: u32,
    entities: &'a Entities,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entities) -> Self {
        Self {
            entities,
            map: 0,
            type_ids: vec![],
        }
    }

    pub fn with_component<T: Any>(&mut self) -> Result<&mut Self> {
        let type_id = TypeId::of::<T>();
        if let Some(bit_mask) = self.entities.get_bitmask(&type_id) {
            self.map |= bit_mask;
            self.type_ids.push(type_id);
        } else {
            return Err(CustomErrors::ComponentNotRegistered.into());
        }
        Ok(self)
    }

    pub fn run(&self) -> Vec<Vec<Rc<RefCell<dyn Any>>>> {
        let indexes: Vec<usize> = self
            .entities
            .map
            .iter()
            .enumerate()
            .filter_map(|(index, entity_map)| {
                if entity_map & self.map == self.map {
                    Some(index)
                } else {
                    None
                }
            })
            .collect();
        let mut result = vec![];

        for type_id in &self.type_ids {
            let entity_components = self.entities.components.get(type_id).unwrap();
            let mut components_to_keep = vec![];
            for index in &indexes {
                components_to_keep.push(entity_components[*index].as_ref().unwrap().clone());
            }
            result.push(components_to_keep);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use core::f32;
    use std::u32;

    use super::*;

    #[test]
    fn query_mask_updating_with_component() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        let mut query = Query::new(&entities);
        query.with_component::<u32>()?.with_component::<f32>()?;

        assert_eq!(query.map, 3);
        assert_eq!(TypeId::of::<u32>(), query.type_ids[0]);
        assert_eq!(TypeId::of::<f32>(), query.type_ids[1]);

        Ok(())
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn run_query() -> Result<()> {
        let mut entities = Entities::default();
        entities.register_component::<u32>();
        entities.register_component::<f32>();
        entities
            .create_entity()
            .with_component(10_u32)?
            .with_component(20.0_f32)?;
        entities.create_entity().with_component(5_u32)?;
        entities.create_entity().with_component(50.0_f32)?;
        entities
            .create_entity()
            .with_component(15_u32)?
            .with_component(25.0_f32)?;
        let mut query = Query::new(&entities);
        query.with_component::<u32>()?.with_component::<f32>()?;

        let query_result = query.run();
        let u32s = &query_result[0];
        let f32s = &query_result[1];

        assert_eq!(u32s.len(), f32s.len());
        assert_eq!(u32s.len(), 2);

        let borrowed_first_u32 = u32s[0].borrow();
        let first_u32 = borrowed_first_u32.downcast_ref::<u32>().unwrap();
        assert_eq!(*first_u32, 10);

        let borrowed_first_f32 = f32s[0].borrow();
        let first_f32 = borrowed_first_f32.downcast_ref::<f32>().unwrap();
        assert_eq!(*first_f32, 20.0);

        let borrowed_second_u32 = u32s[1].borrow();
        let second_u32 = borrowed_second_u32.downcast_ref::<u32>().unwrap();
        assert_eq!(*second_u32, 15);

        let borrowed_second_f32 = f32s[1].borrow();
        let second_f32 = borrowed_second_f32.downcast_ref::<f32>().unwrap();
        assert_eq!(*second_f32, 25.0);

        Ok(())
    }
}
