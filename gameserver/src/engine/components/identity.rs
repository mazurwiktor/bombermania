use specs::{Component, VecStorage};

use engine;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Identity {
    pub id: engine::types::Id
}
