use super::traits::{AreaCalculator, Drawable};

#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

impl AreaCalculator for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radisu{}", self.radius);
    }
}
