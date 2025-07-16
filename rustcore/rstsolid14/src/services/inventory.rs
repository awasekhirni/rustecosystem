use crate::error::ShoppingError;
use crate::interfaces::inventory::InventoryService;
use crate::models::product::Product;
use std::collections::HashMap;

/// Concrete implementation of InventoryService
/// Demonstrates Single Responsibility Principle (S in SOLID)
pub struct InventoryManager {
    products: HashMap<u32, Product>,
}

impl InventoryManager {
    /// Creates a new InventoryManager with empty inventory
    pub fn new() -> Self {
        InventoryManager {
            products: HashMap::new(),
        }
    }
}

impl InventoryService for InventoryManager {
    /// Adds a product to inventory
    fn add_product(&mut self, product: Product) -> Result<(), ShoppingError> {
        if self.products.contains_key(&product.id) {
            Err(ShoppingError::InventoryError(format!(
                "Product with ID {} already exists",
                product.id
            )))
        } else {
            self.products.insert(product.id, product);
            Ok(())
        }
    }

    /// Retrieves a product by ID
    fn get_product(&self, id: u32) -> Result<Product, ShoppingError> {
        self.products.get(&id).cloned().ok_or_else(|| {
            ShoppingError::InventoryError(format!("Product with ID {} not found", id))
        })
    }

    /// Updates stock quantity for a product
    fn update_stock(&mut self, id: u32, quantity: u32) -> Result<(), ShoppingError> {
        if let Some(product) = self.products.get_mut(&id) {
            product.quantity = quantity;
            Ok(())
        } else {
            Err(ShoppingError::InventoryError(format!(
                "Product with ID {} not found",
                id
            )))
        }
    }
}
