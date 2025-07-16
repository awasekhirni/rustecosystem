//! Data models for the hotel booking system

pub mod booking;
pub mod room;

// Re-export the models for easier access
pub use booking::Booking;
pub use room::Room;
