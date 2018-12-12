use std::sync::Mutex;

mod game_state;
mod component;
mod system;

use super::external::call;
use self::game_state::GameState;
use super::geometry;

use self::geometry::{Size, Point};

pub enum Key {
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
}

pub enum KeyState {
    KeyUp,
    KeyDown,
}

lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new(Size {
        width: 1024.0,
        height: 600.0
    }));
}

pub fn update(time: f64) {
    let mut state = &mut GAME_STATE.lock().unwrap();
    system::physics_system(&mut state);
    system::render_system(&state);
}

pub fn resize(size: Size) {
    *GAME_STATE.lock().unwrap() = GameState::new(size);
}

pub fn key_press(key: Key, state: KeyState) {}
