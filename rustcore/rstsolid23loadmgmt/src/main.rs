//! Loan Management System demonstrating SOLID principles in Rust
//!
//! This system manages loan applications, approvals, and notifications
//! while adhering to SOLID design principles.
//! Loan Management System demonstrating SOLID principles in Rust

mod models;
mod repositories;
mod services;
mod traits;

use models::{Customer, LoanApplication};
use repositories::InMemoryLoanRepository;
use services::{EmailNotificationService, LoanService};

fn main() -> anyhow::Result<()> {
    // Initialize dependencies
    let loan_repository = InMemoryLoanRepository::new();
    let notification_service = EmailNotificationService;
    let mut loan_service =
        LoanService::new(Box::new(loan_repository), Box::new(notification_service));

    // Create a customer
    let customer = Customer::new(
        1,
        "John Doe".to_string(),
        "john@example.com".to_string(),
        750,
    );

    // Apply for a loan
    let application = LoanApplication {
        customer_id: customer.id,
        amount: 10_000.0,
        term_months: 12,
        purpose: "Home improvement".to_string(),
    };

    // Process the loan application
    let loan = loan_service.process_application(application)?;
    println!("Loan processed: {:?}", loan);

    // Approve the loan
    let approved_loan = loan_service.approve_loan(loan.id)?;
    println!("Loan approved: {:?}", approved_loan);

    Ok(())
}
