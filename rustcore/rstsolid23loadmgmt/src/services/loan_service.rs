//! Loan service handling business logic

use crate::models::LoanApplication;
use crate::models::loan::{Loan, LoanError};
use crate::traits::{Notifiable, Repository};

pub struct LoanService<R: Repository, N: Notifiable> {
    repository: Box<R>,
    notification_service: Box<N>,
}

impl<R: Repository, N: Notifiable> LoanService<R, N> {
    pub fn new(repository: Box<R>, notification_service: Box<N>) -> Self {
        Self {
            repository,
            notification_service,
        }
    }

    pub fn process_application(&mut self, application: LoanApplication) -> Result<Loan, LoanError> {
        let interest_rate = self.repository.calculate_interest_rate(&application);
        let loan = Loan::from_application(0, application, interest_rate);
        let saved_loan = self.repository.save(loan)?;
        let _ = self.notification_service.notify(&saved_loan);
        Ok(saved_loan)
    }

    pub fn approve_loan(&mut self, id: u32) -> Result<Loan, LoanError> {
        let mut loan = self.repository.find_by_id(id)?;
        loan.approve()?;
        let updated_loan = self.repository.update(loan)?;
        let _ = self.notification_service.notify(&updated_loan);
        Ok(updated_loan)
    }
}
