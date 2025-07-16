//! Service layer for ticket operations
//!
//! Demonstrates Dependency Inversion Principle - depends on TicketRepositoryTrait
//! rather than concrete implementations.

use super::super::models::status::Status;
use super::super::models::ticket::Ticket;
use super::super::traits::repository::TicketRepositoryTrait;

/// Service for ticket-related operations
pub struct TicketService {
    repository: Box<dyn TicketRepositoryTrait>,
}

impl TicketService {
    /// Creates a new TicketService with the given repository
    pub fn new(repository: Box<dyn TicketRepositoryTrait>) -> Self {
        Self { repository }
    }

    /// Creates a new ticket with the given title and description
    pub fn create_ticket(&mut self, title: String, description: String) -> u64 {
        let ticket = Ticket::new(0, title, description); // ID will be set by repository
        self.repository.create(ticket)
    }

    /// Updates a ticket's status
    pub fn update_ticket_status(&mut self, id: u64, status: Status) -> bool {
        self.repository.update_status(id, status)
    }

    /// Retrieves a ticket by ID
    pub fn get_ticket(&self, id: u64) -> Option<Ticket> {
        self.repository.get(id)
    }

    /// Retrieves all tickets
    pub fn get_all_tickets(&self) -> Vec<Ticket> {
        self.repository.get_all()
    }
}
