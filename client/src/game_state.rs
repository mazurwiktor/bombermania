use super::geometry::Size;

pub struct GameState {
    size: Size
}

impl GameState {
    pub fn new(size: Size) -> Self {
        Self{
            size
        }
    }
}