//src/accounts/base_account.rs
//

pub trait Account {
    fn get_id(&self) -> &str;
    fn get_balance(&self) -> f64;
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;

    //Default implementation //interface segregation
    fn transfer_to(&mut self, target: &mut dyn Account, amount: f64) -> Result<(), String> {
        self.withdraw(amount)?;
        target.deposit(amount);
        Ok(())
    }
}
