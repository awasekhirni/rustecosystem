//Traits for Polymorphism with interface like behaviour

pub(crate) trait Drawable {
    fn draw(&self);
}

pub struct Circle {
    pub radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle with radius {}", self.radius);
    }
}

pub struct Square {
    pub side: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing square with side {}", self.side);
    }
}

// let shapes: Vec<Box<dyn Drawable>> = vec![
//     Box::new(Circle { radius: 5.0 }),
//     Box::new(Square { side: 4.0 }),
// ];

// for shape in shapes {
//     shape.draw();
// }
