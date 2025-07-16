use crate::models::booking::BookingStatus;
use crate::models::{Booking, Room};
use crate::repositories::BookingRepository;
use crate::traits::Notifiable;
use chrono::NaiveDate;
use std::error::Error;
use std::fmt;

pub struct BookingService<T: Notifiable, R: BookingRepository> {
    notification_service: T,
    booking_repository: R,
}

impl<T: Notifiable, R: BookingRepository> BookingService<T, R> {
    pub fn new(booking_repository: R, notification_service: T) -> Self {
        BookingService {
            notification_service,
            booking_repository,
        }
    }

    pub fn create_booking(
        &mut self,
        room: Room,
        guest_name: String,
        check_in_date: String,
        check_out_date: String,
    ) -> Result<Booking, Box<dyn Error>> {
        let check_in = NaiveDate::parse_from_str(&check_in_date, "%Y-%m-%d")?;
        let check_out = NaiveDate::parse_from_str(&check_out_date, "%Y-%m-%d")?;

        if check_out <= check_in {
            return Err(Box::new(BookingError::InvalidDateRange));
        }

        let booking_id = self.booking_repository.generate_id();
        let booking = Booking::new(booking_id, room.id, guest_name, check_in, check_out);

        self.booking_repository.save(&booking)?;
        self.notification_service
            .send_confirmation(&booking, &room)?;

        Ok(booking)
    }

    pub fn cancel_booking(&mut self, booking_id: u32) -> Result<(), Box<dyn Error>> {
        let mut booking = self.booking_repository.find_by_id(booking_id)?;
        booking.status = BookingStatus::Cancelled;
        self.booking_repository.save(&booking)?;
        self.notification_service.send_cancellation(&booking)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum BookingError {
    InvalidDateRange,
    NotFound,
}

impl fmt::Display for BookingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BookingError::InvalidDateRange => {
                write!(f, "Check-out date must be after check-in date")
            }
            BookingError::NotFound => write!(f, "Booking not found"),
        }
    }
}

impl Error for BookingError {}
