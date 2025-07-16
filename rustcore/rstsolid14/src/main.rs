//! Online Shopping System demonstrating SOLID principles in Rust
//!
//! This example shows:
//! - S: Single Responsibility Principle (each struct/trait has one responsibility)
//! - O: Open/Closed Principle (open for extension, closed for modification)
//! - L: Liskov Substitution Principle (interchangeable payment processors)
//! - I: Interface Segregation Principle (small focused traits)
//! - D: Dependency Inversion Principle (depend on abstractions not concretions)

mod error;
mod interfaces;
mod models;
mod services;

use error::ShoppingError;
use interfaces::{inventory::InventoryService, payment::PaymentService};
use models::{order::Order, product::Product};
use services::{
    inventory::InventoryManager,
    payment::{CreditCardProcessor, PayPalProcessor},
};

/// Main function demonstrating the shopping system
fn main() -> Result<(), ShoppingError> {
    // Initialize services
    let mut inventory = InventoryManager::new();
    let credit_card_processor = CreditCardProcessor::new("secret-api-key".to_string());
    let paypal_processor = PayPalProcessor::new("merchant@example.com".to_string());

    // Add some products to inventory
    let laptop = Product::new(1, "Laptop".to_string(), 999.99, 10);
    let mouse = Product::new(2, "Mouse".to_string(), 25.50, 50);

    inventory.add_product(laptop.clone())?;
    inventory.add_product(mouse.clone())?;

    // Create an order
    let mut order = Order::new(1);

    // Add products to order (with quantity 1 for simplicity)
    let laptop_for_order = Product::new(laptop.id, laptop.name, laptop.price, 1);
    let mouse_for_order = Product::new(mouse.id, mouse.name, mouse.price, 2);

    order.add_product(laptop_for_order);
    order.add_product(mouse_for_order);

    println!("Order {} total: ${:.2}", order.id, order.total);

    // Process payment with credit card (can be swapped with paypal_processor)
    process_payment(&credit_card_processor, order.total)?;
    // process_payment(&paypal_processor, order.total)?;

    // Update inventory
    inventory.update_stock(laptop.id, laptop.quantity - 1)?;
    inventory.update_stock(mouse.id, mouse.quantity - 2)?;

    println!("Order processed successfully!");
    Ok(())
}

/// Helper function demonstrating Dependency Inversion
/// It depends on the PaymentService trait, not concrete implementations
fn process_payment(processor: &impl PaymentService, amount: f64) -> Result<(), ShoppingError> {
    processor.process_payment(amount)
}
