//! Notification trait (Interface Segregation Principle)

use crate::models::Loan;

/// Notification interface for sending loan status updates
/// This demonstrates the Interface Segregation Principle (I in SOLID)
/// Clients shouldn't be forced to depend on interfaces they don't use
pub trait Notifiable {
    /// Sends a notification about loan status change
    fn notify(&self, loan: &Loan) -> Result<(), String>;
}
