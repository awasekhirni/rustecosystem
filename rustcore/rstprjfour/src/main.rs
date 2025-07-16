use std::f64::consts::PI;

// A base trait for printing
trait Printable {
    fn print(&self) {
        println!("This is a printable object.");
    }
}

// Shape trait inherits from Printable
trait Shape: Printable {
    fn area(&self) -> f64;

    // Default implementation
    fn name(&self) -> &str {
        "Unknown Shape"
    }
}

// Structs
struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

// Implement Printable for Circle and Rectangle
impl Printable for Circle {}
impl Printable for Rectangle {}

// Implement Shape for Circle
impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }
}

// Implement Shape for Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        "Rectangle"
    }
}

// Enum used to decide what shape to create
enum ShapeType {
    Circle(f64),
    Rectangle(f64, f64),
}

// Factory function to create shapes
fn shape_factory(shape_type: ShapeType) -> Box<dyn Shape> {
    match shape_type {
        ShapeType::Circle(r) => Box::new(Circle { radius: r }),
        ShapeType::Rectangle(w, h) => Box::new(Rectangle {
            width: w,
            height: h,
        }),
    }
}

// Generic function using trait bounds
fn describe_shape<T: Shape>(shape: &T) {
    shape.print(); // from Printable
    println!("{} has area {:.2}", shape.name(), shape.area());
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        shape_factory(ShapeType::Circle(2.5)),
        shape_factory(ShapeType::Rectangle(4.0, 3.0)),
    ];

    println!("--- Using trait objects ---");
    for shape in &shapes {
        println!("{} has area {:.2}", shape.name(), shape.area());
    }

    println!("\n--- Using generics with trait bounds ---");
    let my_circle = Circle { radius: 5.0 };
    describe_shape(&my_circle);
}
