use eyre::Result;
use improve_skills_by_building_ecs_library_in_rust::World;

#[test]
fn create_entity() -> Result<()> {
    let mut world = World::new();
    world.register_component::<Location>();
    world.register_component::<Size>();

    world
        .create_entity()
        .with_component(Location(42.0, 24.0))?
        .with_component(Size(10.0))?;
    Ok(())
}

struct Location(pub f32, pub f32);
struct Size(pub f32);
