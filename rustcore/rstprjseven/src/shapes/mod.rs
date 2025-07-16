//project/src/shapes/mod.rs

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

//re-export the modules
pub mod circle;
pub mod rectangle;
