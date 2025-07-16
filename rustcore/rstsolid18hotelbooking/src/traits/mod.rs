//! Traits (interfaces) for the application
//! Demonstrates Interface Segregation Principle - small focused interfaces

pub mod bookable;
pub mod notifiable;

// Re-export traits for easier access
pub use bookable::Bookable;
pub use notifiable::Notifiable;
