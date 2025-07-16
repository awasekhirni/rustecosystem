use super::traits::{AreaCalculator, Drawable};

#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl AreaCalculator for Rectangle {
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {} x {}", self.width, self.height);
    }
}
