use crate::models::product::Product;

/// Order model representing a customer order
#[derive(Debug)]
pub struct Order {
    pub id: u32,
    pub products: Vec<Product>,
    pub total: f64,
    pub is_paid: bool,
}

impl Order {
    /// Creates a new order instance
    pub fn new(id: u32) -> Self {
        Order {
            id,
            products: Vec::new(),
            total: 0.0,
            is_paid: false,
        }
    }

    /// Adds a product to the order and updates the total
    pub fn add_product(&mut self, product: Product) {
        self.products.push(product.clone());
        self.total += product.price * product.quantity as f64;
    }
}
