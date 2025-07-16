//! Notification service implementations

use crate::models::loan::Loan;
use crate::traits::Notifiable;

pub struct EmailNotificationService;

impl Notifiable for EmailNotificationService {
    fn notify(&self, loan: &Loan) -> Result<(), String> {
        let status = match loan.status {
            crate::models::loan::LoanStatus::Pending => "is pending review",
            crate::models::loan::LoanStatus::Approved => "has been approved",
            crate::models::loan::LoanStatus::Rejected => "has been rejected",
            crate::models::loan::LoanStatus::Paid => "has been fully paid",
            crate::models::loan::LoanStatus::Defaulted => "is in default",
        };

        println!(
            "Email notification: Loan #{} for ${:.2} {}",
            loan.id, loan.amount, status
        );

        Ok(())
    }
}
