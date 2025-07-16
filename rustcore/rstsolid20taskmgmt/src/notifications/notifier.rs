//! Notification functionality
//!
//! Demonstrates Interface Segregation Principle - minimal notification interface

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("Notification failed")]
    Failed,
}

/// Minimal notifier interface
pub trait Notifier {
    fn send_notification(&self, to: &str, message: &str) -> Result<(), NotificationError>;
}

/// Console notifier implementation (for demonstration)
pub struct ConsoleNotifier;

impl Notifier for ConsoleNotifier {
    fn send_notification(&self, to: &str, message: &str) -> Result<(), NotificationError> {
        println!("Notification to {}: {}", to, message);
        Ok(())
    }
}
