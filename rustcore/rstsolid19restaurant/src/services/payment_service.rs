// src/services/payment_service.rs
use std::error::Error;

/// Service for handling payments
///
/// Demonstrates Single Responsibility Principle (SRP) -
/// only handles payment processing
pub struct PaymentService;

use crate::traits::OrderProcessor;

impl OrderProcessor for PaymentService {
    fn process_payment(&self, amount: f64) -> Result<(), Box<dyn Error>> {
        println!("Processing payment of ${:.2}", amount);
        Ok(())
    }
}

impl PaymentService {
    pub fn new() -> Self {
        PaymentService
    }

    /// Processes a payment for the given amount
    pub fn process_payment(&self, amount: f64) -> Result<(), Box<dyn Error>> {
        // In a real application, this would integrate with a payment gateway
        println!("Processing payment of ${:.2}", amount);
        Ok(())
    }
}
