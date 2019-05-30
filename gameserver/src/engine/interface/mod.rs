pub mod adapters;
pub mod messages;

#[derive(Debug, PartialEq, Clone)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub fire: bool
}

impl Input {
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            fire: false
        }
    }
}
