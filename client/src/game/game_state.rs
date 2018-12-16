use super::component;
use super::{geometry::Vec2D, Point, Size};
use super::{Input};

type EntityIndex = usize;

pub struct Entity {
    pub physics: Option<component::PhysicsComponent>,
}

pub struct GameState {
    pub size: Size,
    pub entities: Vec<Option<Entity>>,
    pub player: EntityIndex,
    pub input: Option<Input>,
}

impl GameState {
    pub fn new(size: Size) -> Self {
        Self {
            size,
            entities: vec![Option::from(Entity {
                physics: Option::from(component::PhysicsComponent {
                    position: Point::new(size.width / 2.0, size.height / 2.0),
                    velocity: Vec2D::new(0.0, 0.0),
                    speed: 10.0,
                }),
            })],
            player: 0,
            input: None
        }
    }
}
