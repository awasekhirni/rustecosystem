//! Data persistence layer
//!
//! Contains repository implementations for data access

pub mod order_repository;

// Re-export for easier access
pub use order_repository::OrderRepository;
