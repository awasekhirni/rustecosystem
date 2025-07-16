use crate::products::{
    discount::strategy::DiscountStrategy, product::Product, repository::ProductRepository,
};
use std::error::Error;

/// ProductService handles business logic for products
/// Follows Single Responsibility Principle by focusing only on product operations
pub struct ProductService<'a, R: ProductRepository, D: DiscountStrategy> {
    repository: &'a mut R,
    discount_strategy: D,
}

impl<'a, R: ProductRepository, D: DiscountStrategy> ProductService<'a, R, D> {
    pub fn new(repository: &'a mut R, discount_strategy: D) -> Self {
        ProductService {
            repository,
            discount_strategy,
        }
    }

    /// Adds a new product after applying any discounts
    pub fn add_product(&mut self, product: Product) -> Result<(), Box<dyn Error>> {
        let discounted_price = self.discount_strategy.apply_discount(&product);
        let discounted_product = Product {
            price: discounted_price,
            ..product
        };
        self.repository.add(discounted_product)
    }

    /// Gets a product by ID
    pub fn get_product(&self, id: u32) -> Option<&Product> {
        self.repository.get(id)
    }

    /// Lists all products
    pub fn list_products(&self) -> Vec<&Product> {
        self.repository.get_all()
    }

    /// Removes a product
    pub fn remove_product(&mut self, id: u32) -> Result<(), Box<dyn Error>> {
        self.repository.remove(id)
    }
}
