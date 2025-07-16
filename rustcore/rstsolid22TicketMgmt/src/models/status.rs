//! Ticket status model
//!
//! Demonstrates Open/Closed Principle - we can add new status variants
//! without modifying existing code that uses the Status enum.

/// Possible statuses for a ticket
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Open
    }
}
