//! Customer model representing a loan applicant

/// Represents a customer who can apply for loans
#[derive(Debug, Clone)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub credit_score: u32,
}

impl Customer {
    /// Creates a new Customer instance
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the customer
    /// * `name` - Full name of the customer
    /// * `email` - Contact email address
    /// * `credit_score` - Credit score (300-850)
    pub fn new(id: u32, name: String, email: String, credit_score: u32) -> Self {
        Self {
            id,
            name,
            email,
            credit_score,
        }
    }
}
