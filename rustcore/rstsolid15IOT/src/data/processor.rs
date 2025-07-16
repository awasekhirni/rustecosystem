//! Data processing functionality
//!
//! Demonstrates:
//! - SRP: Only handles data transformation
//! - OCP: Processing logic can be extended by providing different closures
//! - DIP: Depends on function trait rather than concrete implementation

/// Processes IoT device data with customizable transformation
pub struct DataProcessor {
    transform: Box<dyn Fn(f64) -> f64>,
}

impl DataProcessor {
    /// Creates a new DataProcessor with the given transformation
    pub fn new<F>(transform: F) -> Self
    where
        F: Fn(f64) -> f64 + 'static,
    {
        Self {
            transform: Box::new(transform),
        }
    }

    /// Processes the input value using the configured transformation
    pub fn process(&self, value: f64) -> f64 {
        (self.transform)(value)
    }
}

// Example of extending functionality without modifying existing code (OCP)
impl DataProcessor {
    /// Creates a processor that adds logging to the transformation
    pub fn with_logging<F>(transform: F) -> Self
    where
        F: Fn(f64) -> f64 + 'static,
    {
        Self::new(move |value| {
            let result = transform(value);
            println!("Processing data: {} -> {}", value, result);
            result
        })
    }
}
