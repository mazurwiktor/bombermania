#[macro_use]
extern crate lazy_static;

mod geometry;
mod game_state;
mod game;
mod external;

use std::os::raw::c_double;

#[no_mangle]
pub extern "C" fn update(time: c_double) {
    game::update(time);
}

#[no_mangle]
pub extern "C" fn resize(width: c_double, height: c_double) {
    game::resize(geometry::Size{width, height});
}

#[no_mangle]
pub extern "C" fn left_keyup() {
    game::key_press(game::Key::LeftArrow, game::KeyState::KeyUp);
}

#[no_mangle]
pub extern "C" fn left_keydown() {
    game::key_press(game::Key::LeftArrow, game::KeyState::KeyDown);
}

#[no_mangle]
pub extern "C" fn right_keyup() {
    game::key_press(game::Key::RightArrow, game::KeyState::KeyUp);
}

#[no_mangle]
pub extern "C" fn right_keydown() {
    game::key_press(game::Key::RightArrow, game::KeyState::KeyDown);
}

#[no_mangle]
pub extern "C" fn up_keyup() {
    game::key_press(game::Key::UpArrow, game::KeyState::KeyUp);
}

#[no_mangle]
pub extern "C" fn up_keydown() {
    game::key_press(game::Key::UpArrow, game::KeyState::KeyDown);
}

#[no_mangle]
pub extern "C" fn down_keyup() {
    game::key_press(game::Key::DownArrow, game::KeyState::KeyUp);
}

#[no_mangle]
pub extern "C" fn down_keydown() {
    game::key_press(game::Key::DownArrow, game::KeyState::KeyDown);
}
