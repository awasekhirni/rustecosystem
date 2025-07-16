//! Repository trait defining data access operations
//!
//! Demonstrates Dependency Inversion Principle - high-level modules
//! depend on this abstraction rather than concrete implementations.

use crate::models::status::Status;
use crate::models::ticket::Ticket;

/// Trait defining operations for ticket persistence
///
/// This allows different repository implementations (in-memory, database, etc)
/// while keeping the service layer unchanged (Open/Closed Principle)
pub trait TicketRepositoryTrait {
    /// Creates a new ticket and returns its ID
    fn create(&mut self, ticket: Ticket) -> u64;

    /// Gets a ticket by ID, returns None if not found
    fn get(&self, id: u64) -> Option<Ticket>;

    /// Updates a ticket's status
    fn update_status(&mut self, id: u64, status: Status) -> bool;

    /// Gets all tickets
    fn get_all(&self) -> Vec<Ticket>;
}
