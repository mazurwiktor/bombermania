use std::os::raw::{c_double};

use super::geometry::Point;

extern "C" {
    fn clear_screen();
    fn draw_player(x: c_double, x: c_double);
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
}