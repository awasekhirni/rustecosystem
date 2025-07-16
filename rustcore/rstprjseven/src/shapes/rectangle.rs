//rstprjseven/src/shapes/rectangle.rs
//imports
use super::Shape;

pub struct Rectangle {
    width: f64,
    height: f64,
}

//methods
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    //additional methods specific to Rectangle
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
