// rstprjseven/src/main.rs
mod shapes;
use shapes::{Shape, circle::Circle, rectangle::Rectangle};

fn main() {
    println!("Example Demo - Project Seven - Awase Khirni Syed-Rust Object Oriented Programming!");
    //create instances
    let rect = Rectangle::new(5.0, 10.0);
    let circle = Circle::new(7.0);

    // Use common interface
    print_shape_info(&rect);
    print_shape_info(&circle);

    // Use Rectangle-specific method
    println!("Is the rectangle a square? {}", rect.is_square());
}

fn print_shape_info(shape: &dyn Shape) {
    println!("Area: {:.2}", shape.area());
    println!("Perimeter: {:.2}\n", shape.perimeter());
}
