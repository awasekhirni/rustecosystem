use crate::products::product::Product;
use std::error::Error;

/// Repository trait defining the interface for product storage
/// This follows the Dependency Inversion Principle (DIP) by depending on abstraction
pub trait ProductRepository {
    fn add(&mut self, product: Product) -> Result<(), Box<dyn Error>>;
    fn get(&self, id: u32) -> Option<&Product>;
    fn get_all(&self) -> Vec<&Product>;
    fn remove(&mut self, id: u32) -> Result<(), Box<dyn Error>>;
}

/// In-memory implementation of ProductRepository
/// This is a concrete implementation that could be replaced with a database version
pub struct InMemoryProductRepository {
    products: Vec<Product>,
}

impl InMemoryProductRepository {
    pub fn new() -> Self {
        InMemoryProductRepository {
            products: Vec::new(),
        }
    }
}

impl ProductRepository for InMemoryProductRepository {
    fn add(&mut self, product: Product) -> Result<(), Box<dyn Error>> {
        self.products.push(product);
        Ok(())
    }

    fn get(&self, id: u32) -> Option<&Product> {
        self.products.iter().find(|&p| p.id == id)
    }

    fn get_all(&self) -> Vec<&Product> {
        self.products.iter().collect()
    }

    fn remove(&mut self, id: u32) -> Result<(), Box<dyn Error>> {
        self.products.retain(|p| p.id != id);
        Ok(())
    }
}
