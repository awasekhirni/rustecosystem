//! Repository trait for data persistence

use crate::models::loan::{Loan, LoanApplication, LoanError};

/// Generic repository trait for loan operations
pub trait Repository {
    fn save(&mut self, loan: Loan) -> Result<Loan, LoanError>;
    fn find_by_id(&self, id: u32) -> Result<Loan, LoanError>;
    fn update(&mut self, loan: Loan) -> Result<Loan, LoanError>;
    fn calculate_interest_rate(&self, application: &LoanApplication) -> f64;
}
