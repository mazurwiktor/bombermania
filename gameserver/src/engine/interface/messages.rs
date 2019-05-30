use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub grid_x: u32,  // XXX: temporary, replace with real board
    pub grid_y: u32
}
