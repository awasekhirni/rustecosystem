use crate::error::ShoppingError;
use crate::models::product::Product;

/// InventoryService trait defining the contract for inventory operations
/// This demonstrates the Dependency Inversion Principle (D in SOLID)
pub trait InventoryService {
    fn add_product(&mut self, product: Product) -> Result<(), ShoppingError>;
    fn get_product(&self, id: u32) -> Result<Product, ShoppingError>;
    fn update_stock(&mut self, id: u32, quantity: u32) -> Result<(), ShoppingError>;
}
