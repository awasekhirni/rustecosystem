/// Custom error types for the shopping system
use std::fmt;

#[derive(Debug)]
pub enum ShoppingError {
    InventoryError(String),
    PaymentError(String),
    OrderError(String),
}

impl fmt::Display for ShoppingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShoppingError::InventoryError(msg) => write!(f, "Inventory Error: {}", msg),
            ShoppingError::PaymentError(msg) => write!(f, "Payment Error: {}", msg),
            ShoppingError::OrderError(msg) => write!(f, "Order Error: {}", msg),
        }
    }
}

impl std::error::Error for ShoppingError {}
