/// Product model representing items in the inventory
#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

impl Product {
    /// Creates a new product instance
    pub fn new(id: u32, name: String, price: f64, quantity: u32) -> Self {
        Product {
            id,
            name,
            price,
            quantity,
        }
    }
}
