use crate::accounts::Account;

pub struct Transaction {
    id: String,
    amount: f64,
}

impl Transaction {
    pub fn new(id: &str, amount: f64) -> Self {
        Transaction {
            id: id.to_string(),
            amount,
        }
    }

    // Dependency Inversion - depends on abstraction (Account trait)
    pub fn execute(&self, account: &mut dyn Account) {
        if self.amount > 0.0 {
            account.deposit(self.amount);
        } else {
            let _ = account.withdraw(-self.amount);
        }
        println!("Executed transaction {} for {}", self.id, account.get_id());
    }
}
