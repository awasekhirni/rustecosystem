use crate::error::ShoppingError;
use crate::interfaces::payment::PaymentService;

/// Concrete implementation of PaymentService using a credit card processor
/// Demonstrates Open/Closed Principle (O in SOLID) - can be extended without modification
pub struct CreditCardProcessor {
    pub api_key: String,
}

impl CreditCardProcessor {
    /// Creates a new CreditCardProcessor with the given API key
    pub fn new(api_key: String) -> Self {
        CreditCardProcessor { api_key }
    }
}

impl PaymentService for CreditCardProcessor {
    /// Processes a payment with the credit card processor
    fn process_payment(&self, amount: f64) -> Result<(), ShoppingError> {
        // Simulate payment processing
        if amount <= 0.0 {
            Err(ShoppingError::PaymentError(
                "Amount must be greater than zero".to_string(),
            ))
        } else if self.api_key.is_empty() {
            Err(ShoppingError::PaymentError("Invalid API key".to_string()))
        } else {
            println!(
                "Processing payment of ${:.2} via CreditCardProcessor",
                amount
            );
            Ok(())
        }
    }
}

/// Another implementation of PaymentService for PayPal
/// Demonstrates Liskov Substitution Principle (L in SOLID) - can be substituted for CreditCardProcessor
pub struct PayPalProcessor {
    pub email: String,
}

impl PayPalProcessor {
    /// Creates a new PayPalProcessor with the given email
    pub fn new(email: String) -> Self {
        PayPalProcessor { email }
    }
}

impl PaymentService for PayPalProcessor {
    /// Processes a payment with PayPal
    fn process_payment(&self, amount: f64) -> Result<(), ShoppingError> {
        // Simulate PayPal payment
        if amount <= 0.0 {
            Err(ShoppingError::PaymentError(
                "Amount must be greater than zero".to_string(),
            ))
        } else if !self.email.contains('@') {
            Err(ShoppingError::PaymentError("Invalid email".to_string()))
        } else {
            println!("Processing payment of ${:.2} via PayPal", amount);
            Ok(())
        }
    }
}
