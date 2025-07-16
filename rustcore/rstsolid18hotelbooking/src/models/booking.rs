use chrono::{Duration, NaiveDate, Utc};
use std::fmt;

/// Represents a booking in the hotel system
/// Demonstrates Single Responsibility Principle - only handles booking data
#[derive(Debug, Clone)]
pub struct Booking {
    pub id: u32,
    pub room_id: u32,
    pub guest_name: String,
    pub check_in_date: NaiveDate,
    pub check_out_date: NaiveDate,
    pub status: BookingStatus,
    pub created_at: chrono::DateTime<Utc>,
}

/// Status of the booking
#[derive(Debug, Clone, PartialEq)]
pub enum BookingStatus {
    Confirmed,
    Cancelled,
    Completed,
}

impl Booking {
    /// Creates a new Booking instance
    pub fn new(
        id: u32,
        room_id: u32,
        guest_name: String,
        check_in_date: NaiveDate,
        check_out_date: NaiveDate,
    ) -> Self {
        Booking {
            id,
            room_id,
            guest_name,
            check_in_date,
            check_out_date,
            status: BookingStatus::Confirmed,
            created_at: Utc::now(),
        }
    }

    /// Calculates the total price for the booking
    pub fn calculate_total_price(&self, price_per_night: f64) -> f64 {
        let duration = self.check_out_date - self.check_in_date;
        price_per_night * duration.num_days() as f64
    }
}

impl fmt::Display for Booking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Booking #{} - Room {} for {} ({} to {})",
            self.id, self.room_id, self.guest_name, self.check_in_date, self.check_out_date
        )
    }
}
