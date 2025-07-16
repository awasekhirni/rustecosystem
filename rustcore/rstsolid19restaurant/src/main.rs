//! Restaurant Order Application demonstrating SOLID principles in Rust
//!
//! This application models a restaurant order system with:
//! - Menu items
//! - Order creation and processing
//! - Payment handling
//! - Data persistence

mod models;
mod repositories;
mod services;
mod traits;

use models::{MenuItem, Order};
use repositories::OrderRepository;
use services::{OrderService, PaymentService};
use std::boxed::Box;

fn main() {
    // Initialize dependencies
    // Initialize dependencies
    // src/main.rs
    let order_repository = Box::new(OrderRepository::new());
    let payment_service = Box::new(PaymentService::new());

    // Create order service with dependencies injected
    let mut order_service = OrderService::new(order_repository, payment_service);

    // Create some menu items
    let steak = MenuItem::new("Ribeye Steak", 24.99);
    let salad = MenuItem::new("Caesar Salad", 8.99);

    // Create a new order
    let mut order = Order::new(1);
    order.add_item(steak);
    order.add_item(salad);

    // Process the order
    match order_service.process_order(order) {
        Ok(processed_order) => {
            println!("Order processed successfully!");
            println!("Total: ${:.2}", processed_order.total_amount());
            println!("Status: {:?}", processed_order.status());
        }
        Err(e) => println!("Error processing order: {}", e),
    }
}
