use specs::{Builder, Entity, World};

use engine::components::physics::{Position};

pub fn create_player(world: &mut World, x: u32, y: u32) -> Entity {
    world.create_entity()
        .with(Position{ x: x, y: y })
        .build()
}
