use std::sync::Mutex;

use super::external::call;
use super::game_state::GameState;
use super::geometry::{Point, Size};

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
    static ref GAME: Mutex<Game> = Mutex::new(Game::new(Size {
        width: 1024.0,
        height: 600.0
    }));
}

struct Game {
    state: GameState,
}

impl Game {
    fn new(size: Size) -> Self {
        Self {
            state: GameState::new(size),
        }
    }
}

pub fn update(time: f64) {
    call::clear_screen();
    call::draw_player(Point::new(1.0, 1.0));
}

pub fn resize(size: Size) {
    *GAME.lock().unwrap() = Game::new(size);
}

pub fn key_press(key: Key, state: KeyState) {}
