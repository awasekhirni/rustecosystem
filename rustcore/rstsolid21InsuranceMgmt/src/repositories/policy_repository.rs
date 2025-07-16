use crate::models::policy::Policy;
use crate::traits::Repository;
use std::collections::HashMap;

pub struct PolicyRepository {
    policies: HashMap<u32, Policy>,
    next_id: u32,
}

impl PolicyRepository {
    pub fn new() -> Self {
        PolicyRepository {
            policies: HashMap::new(),
            next_id: 1,
        }
    }
}

impl Repository<Policy, u32> for PolicyRepository {
    fn get_all(&self) -> Vec<Policy> {
        self.policies.values().cloned().collect()
    }

    fn get_by_id(&self, id: u32) -> Option<Policy> {
        self.policies.get(&id).cloned()
    }

    fn add(&mut self, mut policy: Policy) -> u32 {
        let id = self.next_id;
        policy.id = id;
        self.policies.insert(id, policy);
        self.next_id += 1;
        id
    }

    fn update(&mut self, id: u32, policy: Policy) -> bool {
        if self.policies.contains_key(&id) {
            self.policies.insert(id, policy);
            true
        } else {
            false
        }
    }

    fn delete(&mut self, id: u32) -> bool {
        self.policies.remove(&id).is_some()
    }
}
