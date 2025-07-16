//src/accounts/checking_account.rs

use super::Account;

pub struct CheckingAccount {
    id: String,
    balance: f64,
    overdraft_limit: f64,
}

impl CheckingAccount {
    pub fn new(id: &str, balance: f64, overdraft_limit: f64) -> Self {
        CheckingAccount {
            id: id.to_string(),
            balance,
            overdraft_limit,
        }
    }
}

impl Account for CheckingAccount {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    // Liskov Substitution - can be used interchangeably with SavingsAccount
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance + self.overdraft_limit >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Exceeds overdraft limit".to_string())
        }
    }
}
