//! Service layer for business logic
//! Demonstrates Single Responsibility Principle - each service handles specific business logic

pub mod booking_service;
pub mod notification_service;

// Re-export services for easier access
pub use booking_service::BookingService;
pub use notification_service::NotificationService;
