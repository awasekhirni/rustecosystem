use crate::models::Booking;
use std::collections::HashMap;
use std::error::Error;
use std::sync::atomic::{AtomicU32, Ordering};

pub trait BookingRepository {
    fn save(&mut self, booking: &Booking) -> Result<(), Box<dyn Error>>;
    fn find_by_id(&self, id: u32) -> Result<Booking, Box<dyn Error>>;
    fn generate_id(&self) -> u32;
}

pub struct InMemoryBookingRepository {
    bookings: HashMap<u32, Booking>,
    id_counter: AtomicU32,
}

impl InMemoryBookingRepository {
    pub fn new() -> Self {
        InMemoryBookingRepository {
            bookings: HashMap::new(),
            id_counter: AtomicU32::new(1),
        }
    }
}

impl BookingRepository for InMemoryBookingRepository {
    fn save(&mut self, booking: &Booking) -> Result<(), Box<dyn Error>> {
        self.bookings.insert(booking.id, booking.clone());
        Ok(())
    }

    fn find_by_id(&self, id: u32) -> Result<Booking, Box<dyn Error>> {
        Ok(self
            .bookings
            .get(&id)
            .cloned()
            .ok_or_else(|| Box::new(crate::services::booking_service::BookingError::NotFound))?)
    }

    fn generate_id(&self) -> u32 {
        self.id_counter.fetch_add(1, Ordering::SeqCst)
    }
}
