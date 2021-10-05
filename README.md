improve_skills_by_building_ecs_library_in_rust

# Release Notes

## 1.0.2

Fixed a bug in the Entities where if the user deleted a component twice it would re-add the component.

## 1.0.1

A community member found a possible bug in the query. We wrote a test to verify the bug but couldn't replicate it.

That tests is remaining inside the [query.rs](src/entities/query.rs) file that is verifying that the query is only getting entities that have the components that are being queried for.

# Contributing

Feel free to pull request or add issues for anything. I have a vision for this project that it will be relatively simple and used for teaching purposes so if you're not sure if I would accept the PR please start a conversation in the issues.