extern crate euclid;

pub type Point = euclid::TypedPoint2D<f64, euclid::UnknownUnit>;
pub type Vec2D = euclid::TypedVector2D<f64, euclid::UnknownUnit>;

#[derive(Copy, Clone)]
pub struct Size {
    pub width: f64,
    pub height: f64
}