//! Ticket model representing a support ticket in the system
//!
//! Demonstrates Single Responsibility Principle - this struct is only
//! responsible for representing ticket data.

use super::status::Status;
use std::time::SystemTime;

/// A support ticket with title, description, status, and timestamps
#[derive(Debug, Clone)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl Ticket {
    /// Creates a new ticket with the given title and description
    pub fn new(id: u64, title: String, description: String) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            title,
            description,
            status: Status::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates the ticket status and sets the updated_at timestamp
    pub fn update_status(&mut self, new_status: Status) {
        self.status = new_status;
        self.updated_at = SystemTime::now();
    }
}
