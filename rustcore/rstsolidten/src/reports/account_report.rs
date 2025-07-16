use crate::accounts::Account;

pub struct AccountReport;

impl AccountReport {
    // Single Responsibility - only handles reporting
    pub fn generate(&self, account: &dyn Account) {
        println!(
            "Account Report\nID: {}\nBalance: {:.2}\n",
            account.get_id(),
            account.get_balance()
        );
    }
}
