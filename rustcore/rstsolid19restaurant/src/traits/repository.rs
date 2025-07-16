use std::error::Error;

/// Generic repository trait
///
/// Demonstrates Liskov Substitution Principle (LSP) -
/// any implementation can be substituted for another
pub trait Repository<T> {
    fn save(&mut self, entity: &T) -> Result<(), Box<dyn Error>>;
    fn find_by_id(&self, id: u64) -> Result<Option<T>, Box<dyn Error>>;
}
