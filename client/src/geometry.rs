extern crate euclid;

pub type Point = euclid::TypedPoint2D<f64, euclid::UnknownUnit>;

pub struct Size {
    pub width: f64,
    pub height: f64
}