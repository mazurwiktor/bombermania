use std::os::raw::{c_double, c_int};

extern "C" {
    fn clear_screen();
    fn draw_player(x: c_double, x: c_double);
}

#[no_mangle]
pub extern "C" fn update(time: c_double) {
    // game logic goes here
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    clear_screen();
    draw_player(0.0, 0.0);
}

#[no_mangle]
pub extern "C" fn resize(width: c_double, height: c_double) {
    // update size
}

#[no_mangle]
pub extern "C" fn left_keyup() {
}


#[no_mangle]
pub extern "C" fn left_keydown() {
}


#[no_mangle]
pub extern "C" fn right_keyup() {
}


#[no_mangle]
pub extern "C" fn right_keydown() {
}


#[no_mangle]
pub extern "C" fn up_keyup() {
}


#[no_mangle]
pub extern "C" fn up_keydown() {
}


#[no_mangle]
pub extern "C" fn down_keyup() {
}


#[no_mangle]
pub extern "C" fn down_keydown() {
}