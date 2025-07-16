mod accounts;
mod reports;
mod transactions;

use accounts::{Account, CheckingAccount, SavingsAccount};
use reports::AccountReport;
use transactions::Transaction;

fn main() {
    // Create accounts
    let mut savings = SavingsAccount::new("SA001", 1000.0, 0.05);
    let mut checking = CheckingAccount::new("CH001", 500.0, 100.0);

    // Perform transactions
    let deposit = Transaction::new("DEP001", 200.0);
    deposit.execute(&mut savings);

    let withdrawal = Transaction::new("WDR001", 300.0);
    withdrawal.execute(&mut checking);

    // Transfer between accounts (demonstrates dependency inversion)
    savings.transfer_to(&mut checking, 150.0);

    // Generate reports
    let report = AccountReport;
    report.generate(&savings);
    report.generate(&checking);

    // Apply interest to savings (Open/Closed principle)
    savings.apply_interest();
    report.generate(&savings);
}
