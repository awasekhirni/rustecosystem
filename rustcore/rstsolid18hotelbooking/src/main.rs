mod models;
mod repositories;
mod services;
mod traits;

use models::Room;
use repositories::BookingRepository;
use services::{BookingService, NotificationService};
use traits::Notifiable;

fn main() {
    println!("Hotel Booking System");

    let booking_repository = repositories::booking_repository::InMemoryBookingRepository::new();
    let notification_service = NotificationService::new();
    let mut booking_service = BookingService::new(booking_repository, notification_service);

    let room = Room::new(101, "Deluxe", 150.0);

    match booking_service.create_booking(
        room,
        "John Doe".to_string(),
        "2023-01-15".to_string(),
        "2023-01-20".to_string(),
    ) {
        Ok(booking) => {
            println!("Booking created successfully: {:?}", booking);

            if let Err(e) = booking_service.cancel_booking(booking.id) {
                println!("Failed to cancel booking: {}", e);
            }
        }
        Err(e) => println!("Failed to create booking: {}", e),
    }
}
