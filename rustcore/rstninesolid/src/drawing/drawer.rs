//open closed principle open for extension and closed for modificaiton
// dependency inversion principle (DIP)
// depend on abstraction, not concrete implementations
use crate::shapes::Drawable;

pub trait DrawingTool {
    fn draw_shape<T: Drawable>(&mut self, shape: &T); // Changed to &mut self
}

pub struct ConsoleDrawer;

impl DrawingTool for ConsoleDrawer {
    fn draw_shape<T: Drawable>(&mut self, shape: &T) {
        // Changed to &mut self
        shape.draw();
    }
}

#[derive(Default)]
pub struct MockDrawer {
    pub drawn_items: Vec<String>,
}

// impl DrawingTool for MockDrawer {
//     fn draw_shape<T: Drawable>(&mut self, shape: &T) {
//         // Changed to &mut self
//         let debug_output = format!("{:?}", shape);
//         self.drawn_items.push(debug_output);
//     }
// }
