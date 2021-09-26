improve_skills_by_building_ecs_library_in_rust

# Release Notes

## 1.0.1

A community member found a possible bug in the query. We wrote a test to verify the bug but couldn't replicate it.

That tests is remaining inside the [query.rs](src/entities/query.rs) file that is verifying that the query is only getting entities that have the components that are being queried for.
