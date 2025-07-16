//! Traits defining interfaces for the application
//!
//! These abstractions help maintain loose coupling between components

pub mod order_processor;
pub mod repository;

// Re-export for easier access
pub use order_processor::OrderProcessor;
pub use repository::Repository;
