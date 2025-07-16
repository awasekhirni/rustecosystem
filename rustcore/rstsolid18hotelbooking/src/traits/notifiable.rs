use crate::models::{Booking, Room};
use std::error::Error;

/// Trait for notification services
/// Demonstrates Interface Segregation Principle - focused interface
pub trait Notifiable {
    fn send_confirmation(&self, booking: &Booking, room: &Room) -> Result<(), Box<dyn Error>>;
    fn send_cancellation(&self, booking: &Booking) -> Result<(), Box<dyn Error>>;
}
