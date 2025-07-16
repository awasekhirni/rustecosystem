use crate::models::{Booking, Room};
use crate::traits::Notifiable;
use std::error::Error;
use std::fmt;

/// Service for handling notifications
/// Demonstrates Single Responsibility Principle - only handles notifications
pub struct NotificationService;

impl NotificationService {
    pub fn new() -> Self {
        NotificationService
    }
}

impl Notifiable for NotificationService {
    fn send_confirmation(&self, booking: &Booking, room: &Room) -> Result<(), Box<dyn Error>> {
        println!(
            "Sending booking confirmation to {} for room {} (${}/night). Total: ${:.2}",
            booking.guest_name,
            room.id,
            room.price_per_night,
            booking.calculate_total_price(room.price_per_night)
        );
        Ok(())
    }

    fn send_cancellation(&self, booking: &Booking) -> Result<(), Box<dyn Error>> {
        println!(
            "Sending cancellation notification for booking #{} to {}",
            booking.id, booking.guest_name
        );
        Ok(())
    }
}
