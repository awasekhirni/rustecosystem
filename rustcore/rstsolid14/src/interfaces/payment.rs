use crate::error::ShoppingError;

/// PaymentService trait defining the contract for payment processing
/// Another example of Dependency Inversion Principle
pub trait PaymentService {
    fn process_payment(&self, amount: f64) -> Result<(), ShoppingError>;
}
