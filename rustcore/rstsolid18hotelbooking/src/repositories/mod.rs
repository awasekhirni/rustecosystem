//! Data persistence layer
//! Demonstrates:
//! - Single Responsibility Principle (handles data persistence)
//! - Dependency Inversion Principle (repository trait)

pub mod booking_repository;

// Re-export repository for easier access
pub use booking_repository::BookingRepository;
