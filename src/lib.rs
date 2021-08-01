use std::any::Any;

use entities::Entities;
use resource::Resource;

pub mod custom_errors;
mod entities;
mod resource;

#[derive(Default, Debug)]
pub struct World {
    resources: Resource,
    entities: Entities,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a resource to the world so that anyone with access to the world can query for it immutably or mutably. Generally resources are pieces of data that are not associated with individual entities. An example of a resource could be the average fps.
    ///
    /// Resources are stored based on their type id, so we could store one u32 resource. If we want to store more u32's then we can wrap the data in a tuple struct. See the [integration tests](https://github.com/brooks-builds/improve_skills_by_building_ecs_library_in_rust/blob/main/tests/resources.rs) for an example.
    ///
    /// ```
    /// use improve_skills_by_building_ecs_library_in_rust::World;
    /// let mut world = World::new();
    /// world.add_resource(10_u32);
    /// ```
    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }

    /// Query for a resource and get a reference to it. The type of the resource must be added in so that we can find it.
    /// ```
    /// use improve_skills_by_building_ecs_library_in_rust::World;
    /// let mut world = World::new();
    /// world.add_resource(10_u32);
    /// let resource = world.get_resource::<u32>().unwrap();
    /// assert_eq!(*resource, 10);
    /// ```
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /// Query for a resource and get a mutable reference to it. The type of the resource must be added in so that we can find it.
    /// ```
    /// use improve_skills_by_building_ecs_library_in_rust::World;
    /// let mut world = World::new();
    /// world.add_resource(10_u32);
    /// {
    ///   let resource = world.get_resource_mut::<u32>().unwrap();
    ///   *resource += 1;
    /// }
    /// let resource = world.get_resource::<u32>().unwrap();
    /// assert_eq!(*resource, 11);
    /// ```
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /**
    This will remove the resource from the world, and it doesn't care if the resource exists at this point in time.
    */
    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }

    pub fn register_component<T: Any + 'static>(&mut self) {
        self.entities.register_component::<T>();
    }

    pub fn create_entity(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }
}

#[cfg(test)]
mod tests {}
