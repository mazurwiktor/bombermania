use std::os::raw::{c_double, c_char};
use std::ffi::CString;

use super::geometry::Point;

extern "C" {
    fn clear_screen();
    fn draw_player(x: c_double, x: c_double);
    fn print(msg: *const c_char);
}


pub mod call {
    use super::Point;

    pub fn clear_screen() {
        unsafe {
            super::clear_screen();
        }
    }

    pub fn draw_player(point: Point) {
        unsafe {
            super::draw_player(point.x, point.y);
        }
    }

    pub fn print(msg: &str) {
        let c_to_print = super::CString::new(msg).expect("CString::new failed");
        unsafe {
            super::print(c_to_print.as_ptr());
        }
    }
}