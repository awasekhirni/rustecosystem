use std::error::Error;

/// Trait for processing orders
///
/// Demonstrates Interface Segregation Principle (ISP) -
/// focused interface for order processing
pub trait OrderProcessor {
    fn process_payment(&self, amount: f64) -> Result<(), Box<dyn Error>>;
}
