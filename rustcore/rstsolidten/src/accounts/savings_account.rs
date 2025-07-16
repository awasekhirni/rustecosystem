//src/accounts/savings_account
//
use super::Account;

pub struct SavingsAccount {
    id: String,
    balance: f64,
    interest_rate: f64,
}

impl SavingsAccount {
    pub fn new(id: &str, balance: f64, interest_rate: f64) -> Self {
        SavingsAccount {
            id: id.to_string(),
            balance,
            interest_rate,
        }
    }

    //Single Responsibility --interest calculation is only in Savings Account
    pub fn apply_interest(&mut self) {
        let interest = self.balance * self.interest_rate;
        self.deposit(interest);
    }
}

impl Account for SavingsAccount {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    }
}
