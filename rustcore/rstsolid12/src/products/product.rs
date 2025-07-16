/// Product entity representing a sellable item
#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub category: String,
}

impl Product {
    /// Creates a new Product instance
    pub fn new(id: u32, name: String, price: f64, category: String) -> Self {
        Product {
            id,
            name,
            price,
            category,
        }
    }
}
