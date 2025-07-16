//! Service layer for business logic
//!
//! Contains services that coordinate between different components
//! while maintaining separation of concerns (Single Responsibility)

pub mod order_service;
pub mod payment_service;

// Re-export for easier access
pub use order_service::OrderService;
pub use payment_service::PaymentService;
