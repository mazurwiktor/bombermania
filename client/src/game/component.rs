use super::geometry;

pub struct PhysicsComponent {
    pub position: geometry::Point,
    pub velocity: geometry::Vec2D,
    pub speed: f64
}
