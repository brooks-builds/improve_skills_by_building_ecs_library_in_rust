use improve_skills_by_building_ecs_library_in_rust::World;

#[test]
fn create_and_get_resource_immutably() {
    let world = initialize_world();
    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 60)
}

#[test]
fn get_resources_mutably() {
    let mut world = initialize_world();
    {
        let fps: &mut FpsResource = world.get_resource_mut::<FpsResource>().unwrap();
        fps.0 += 1;
    }
    let fps = world.get_resource::<FpsResource>().unwrap();
    assert_eq!(fps.0, 61);
}

#[test]
fn delete_resource() {
    let mut world = initialize_world();
    world.delete_resource::<FpsResource>();
    let deleted_resource = world.get_resource::<FpsResource>();
    assert!(deleted_resource.is_none());
}

fn initialize_world() -> World {
    let mut world = World::new();
    world.add_resource(FpsResource(60));
    world
}

#[derive(Debug)]
struct FpsResource(pub u32);

impl std::ops::Deref for FpsResource {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
