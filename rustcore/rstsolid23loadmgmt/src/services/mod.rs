//! Service layer implementations

mod loan_service;
mod notification_service;

pub use loan_service::LoanService;
pub use notification_service::EmailNotificationService;
