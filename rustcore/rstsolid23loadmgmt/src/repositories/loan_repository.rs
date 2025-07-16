//! In-memory implementation of the loan repository

use crate::models::loan::{Loan, LoanApplication, LoanError};
use crate::traits::Repository;
use std::collections::HashMap;

pub struct InMemoryLoanRepository {
    loans: HashMap<u32, Loan>,
    next_id: u32,
}

impl InMemoryLoanRepository {
    pub fn new() -> Self {
        Self {
            loans: HashMap::new(),
            next_id: 1,
        }
    }
}

impl Repository for InMemoryLoanRepository {
    fn save(&mut self, mut loan: Loan) -> Result<Loan, LoanError> {
        loan.id = self.next_id;
        self.next_id += 1;
        self.loans.insert(loan.id, loan.clone());
        Ok(loan)
    }

    fn find_by_id(&self, id: u32) -> Result<Loan, LoanError> {
        self.loans.get(&id).cloned().ok_or(LoanError::NotFound)
    }

    fn update(&mut self, loan: Loan) -> Result<Loan, LoanError> {
        if !self.loans.contains_key(&loan.id) {
            return Err(LoanError::NotFound);
        }
        self.loans.insert(loan.id, loan.clone());
        Ok(loan)
    }

    fn calculate_interest_rate(&self, application: &LoanApplication) -> f64 {
        let base_rate = 3.5;
        let amount_factor = (application.amount / 10_000.0).min(5.0);
        let term_factor = (application.term_months as f64 / 12.0) * 0.5;

        base_rate + amount_factor + term_factor
    }
}
