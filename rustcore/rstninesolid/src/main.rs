mod drawing;
mod shapes;

//subtypes should be substitutable for their base types
// liskov substitution principle LSP

use drawing::{ConsoleDrawer, DrawingTool};
use shapes::{AreaCalculator, Circle, Drawable, Rectangle};

fn print_area<T: AreaCalculator>(shape: &T) {
    println!("Area: {}", shape.calculate_area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    // Using the concrete implementation
    let console_drawer = ConsoleDrawer;
    console_drawer.draw_shape(&circle);
    console_drawer.draw_shape(&rectangle);

    // Function that works with any DrawingTool
    draw_with_tool(&console_drawer, &circle);
    draw_with_tool(&console_drawer, &rectangle);

    print_area(&circle);
    print_area(&rectangle);
}

fn draw_with_tool(tool: &impl DrawingTool, shape: &impl Drawable) {
    tool.draw_shape(shape);
}
