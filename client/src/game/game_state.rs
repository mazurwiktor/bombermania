use super::{Size, Point, geometry::Vec2D};
use super::component;

type EntityIndex = usize;

pub struct Entity {
    pub physics: Option<component::PhysicsComponent>
}


pub struct GameState {
    pub size: Size,
    pub entities: Vec<Option<Entity>>,
    pub player: EntityIndex
}

impl GameState {
    pub fn new(size: Size) -> Self {
        Self{
            size,
            entities: vec![Option::from(Entity{
                physics: Option::from(component::PhysicsComponent{
                    position: Point::new(size.width/2.0, size.height/2.0),
                    velocity: Vec2D::new(1.0, 0.0),
                    speed: 1.0
                })
            })],
            player: 0
        }
    }
}