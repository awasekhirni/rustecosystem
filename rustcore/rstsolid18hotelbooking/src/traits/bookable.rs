use crate::models::Booking;
use std::error::Error;

/// Trait for bookable items
/// Demonstrates Interface Segregation Principle - focused interface
pub trait Bookable {
    fn create_booking(&self, booking: &Booking) -> Result<(), Box<dyn Error>>;
    fn cancel_booking(&self, booking_id: u32) -> Result<(), Box<dyn Error>>;
}
