//! Ticket Management System demonstrating SOLID principles in Rust
//!
//! SOLID Principles:
//! - S: Single Responsibility Principle (each module/struct has one responsibility)
//! - O: Open/Closed Principle (open for extension, closed for modification)
//! - L: Liskov Substitution Principle (subtypes should be substitutable)
//! - I: Interface Segregation Principle (many client-specific traits)
//! - D: Dependency Inversion Principle (depend on abstractions, not concretions)

mod models;
mod repositories;
mod services;
mod traits;

use models::status::Status;
use repositories::ticket_repository::TicketRepository;
use services::ticket_service::TicketService;

fn main() {
    println!("Ticket Management System");

    // Dependency Injection demonstrating Dependency Inversion Principle
    let repo = TicketRepository::new();
    let mut ticket_service = TicketService::new(Box::new(repo));

    // Create tickets
    let ticket_id = ticket_service.create_ticket(
        "System crash".to_string(),
        "The system crashes when clicking button".to_string(),
    );
    println!("Created ticket with ID: {}", ticket_id);

    // Change status demonstrating Open/Closed Principle
    ticket_service.update_ticket_status(ticket_id, Status::InProgress);
    println!("Updated ticket status to InProgress");

    // Get ticket
    if let Some(ticket) = ticket_service.get_ticket(ticket_id) {
        println!("Retrieved ticket: {:?}", ticket);
    }
}
