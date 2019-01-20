use std::collections::HashMap;

use engine;

/// Holds current keystrokes for every player.
#[derive(Default)]
pub struct InputState {
    pub content: HashMap<engine::types::Id, engine::interface::Input>
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            content: HashMap::new()
        }
    }
}
