/// Policy model with different policy types
/// Demonstrates OCP by using enums that can be extended
#[derive(Debug, Clone)]
pub struct Policy {
    pub id: u32,
    pub customer_id: u32,
    pub policy_type: PolicyType,
    pub premium: f64,
    pub active: bool,
}

/// Policy types - can be extended without modifying existing code
#[derive(Debug, Clone)]
pub enum PolicyType {
    Auto { vehicle_vin: String },
    Home { address: String },
    Life { beneficiary: String },
}

impl Policy {
    /// Create a new policy
    pub fn new(id: u32, customer_id: u32, policy_type: PolicyType, premium: f64) -> Self {
        Policy {
            id,
            customer_id,
            policy_type,
            premium,
            active: true,
        }
    }
}
