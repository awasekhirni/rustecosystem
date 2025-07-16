//Basic Struct with Methods (Encapsulation)
pub struct BankAccount {
    balance: f64,
    owner: String,
}

impl BankAccount {
    // Constructor-like associated function
    pub fn new(owner: String, initial_balance: f64) -> Self {
        BankAccount {
            owner,
            balance: initial_balance,
        }
    }

    // Instance method
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    pub fn get_balance(&self) -> f64 {
        self.balance
    }
}

// let mut account = BankAccount::new("Alice".to_string(), 100.0);
// account.deposit(50.0);
// println!("Balance: {}", account.get_balance());
