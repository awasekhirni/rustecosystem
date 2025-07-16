//! In-memory implementation of TicketRepositoryTrait
//!
//! Demonstrates Liskov Substitution Principle - this can be substituted
//! for any other implementation of TicketRepositoryTrait.

use super::super::models::status::Status;
use super::super::models::ticket::Ticket;
use super::super::traits::repository::TicketRepositoryTrait;
use std::collections::HashMap;

/// In-memory ticket repository using a HashMap
pub struct TicketRepository {
    tickets: HashMap<u64, Ticket>,
    next_id: u64,
}

impl TicketRepository {
    /// Creates a new in-memory repository
    pub fn new() -> Self {
        Self {
            tickets: HashMap::new(),
            next_id: 1,
        }
    }
}

impl TicketRepositoryTrait for TicketRepository {
    fn create(&mut self, ticket: Ticket) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.tickets.insert(id, ticket);
        id
    }

    fn get(&self, id: u64) -> Option<Ticket> {
        self.tickets.get(&id).cloned()
    }

    fn update_status(&mut self, id: u64, status: Status) -> bool {
        if let Some(ticket) = self.tickets.get_mut(&id) {
            ticket.update_status(status);
            true
        } else {
            false
        }
    }

    fn get_all(&self) -> Vec<Ticket> {
        self.tickets.values().cloned().collect()
    }
}
