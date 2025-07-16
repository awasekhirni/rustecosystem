//! Data models for the restaurant application
//!
//! Contains the core domain models like MenuItem and Order

pub mod menu_item;
pub mod order;

// Re-export for easier access
pub use menu_item::MenuItem;
pub use order::{Order, OrderStatus};
