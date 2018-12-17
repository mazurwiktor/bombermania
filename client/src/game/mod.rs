use std::collections::HashSet;
use std::sync::Mutex;

mod component;
mod game_state;
mod system;

use self::game_state::GameState;
use super::external::call;
use super::geometry;

use self::geometry::{Point, Size};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Clone)]
pub struct Input {
    key_pressed: HashSet<Key>,
}

lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new(Size {
        width: 1024.0,
        height: 600.0
    }));
}

pub fn update(time_elapsed: f64) {
    let mut state = &mut GAME_STATE.lock().unwrap();
    state.time_elapsed = time_elapsed;
    system::input_system(&mut state);
    system::physics_system(&mut state);
    system::render_system(&state);
}

pub fn resize(size: Size) {
    *GAME_STATE.lock().unwrap() = GameState::new(size);
}

pub fn key_press(key: Key, key_state: KeyState) {
    let state = &mut GAME_STATE.lock().unwrap();
    let input_state = &state.input;
    let mut input = if let Some(input) = input_state {
        input.clone()
    } else {
        Input {key_pressed: HashSet::new()}
    };

    match key_state {
        KeyState::KeyDown => {
            input.key_pressed.insert(key);
        }
        KeyState::KeyUp => {
            if input.key_pressed.contains(&key) {
                input.key_pressed.remove(&key);
            }
        }
    }

    if input.key_pressed.is_empty() {
        state.input = None
    } else {
        state.input = Some(input);
    }
}
