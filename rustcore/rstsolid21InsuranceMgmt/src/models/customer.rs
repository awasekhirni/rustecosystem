/// Customer model demonstrating OCP by being open for extension
/// through traits but closed for modification
#[derive(Debug, Clone)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub policy_ids: Vec<u32>,
}

impl Customer {
    /// Create a new customer
    pub fn new(id: u32, name: String, email: String) -> Self {
        Customer {
            id,
            name,
            email,
            policy_ids: Vec::new(),
        }
    }

    /// Add a policy reference to the customer
    pub fn add_policy(&mut self, policy_id: u32) {
        self.policy_ids.push(policy_id);
    }
}
