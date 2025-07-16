//traits -- single responsibility principle
//src/shapes/traits.rs
//

pub trait AreaCalculator {
    fn calculate_area(&self) -> f64;
}

pub trait Drawable: std::fmt::Debug {
    fn draw(&self);
}
