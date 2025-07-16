//! Loan-related models and statuses

use thiserror::Error;

/// Possible statuses of a loan
#[derive(Debug, Clone, PartialEq)]
pub enum LoanStatus {
    Pending,
    Approved,
    Rejected,
    Paid,
    Defaulted,
}

/// Represents a loan application
#[derive(Debug, Clone)]
pub struct LoanApplication {
    pub customer_id: u32,
    pub amount: f64,
    pub term_months: u32,
    pub purpose: String,
}

/// Represents an approved loan
#[derive(Debug, Clone)]
pub struct Loan {
    pub id: u32,
    pub customer_id: u32,
    pub amount: f64,
    pub term_months: u32,
    pub purpose: String,
    pub status: LoanStatus,
    pub interest_rate: f64,
}

/// Loan-related errors
#[derive(Error, Debug)]
pub enum LoanError {
    #[error("Loan not found")]
    NotFound,
    #[error("Invalid loan state transition")]
    InvalidStateTransition,
}

impl Loan {
    /// Creates a new Loan from an application
    ///
    /// # Arguments
    /// * `id` - Unique loan identifier
    /// * `application` - The loan application
    /// * `interest_rate` - Determined interest rate
    pub fn from_application(id: u32, application: LoanApplication, interest_rate: f64) -> Self {
        Self {
            id,
            customer_id: application.customer_id,
            amount: application.amount,
            term_months: application.term_months,
            purpose: application.purpose,
            status: LoanStatus::Pending,
            interest_rate,
        }
    }

    /// Approves the loan
    pub fn approve(&mut self) -> Result<(), LoanError> {
        if self.status != LoanStatus::Pending {
            return Err(LoanError::InvalidStateTransition);
        }
        self.status = LoanStatus::Approved;
        Ok(())
    }
}
