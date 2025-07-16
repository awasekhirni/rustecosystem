use crate::models::customer::Customer;
use crate::traits::Repository;
use std::collections::HashMap;

pub struct CustomerRepository {
    customers: HashMap<u32, Customer>,
    next_id: u32,
}

impl CustomerRepository {
    pub fn new() -> Self {
        CustomerRepository {
            customers: HashMap::new(),
            next_id: 1,
        }
    }
}

impl Repository<Customer, u32> for CustomerRepository {
    fn get_all(&self) -> Vec<Customer> {
        self.customers.values().cloned().collect()
    }

    fn get_by_id(&self, id: u32) -> Option<Customer> {
        self.customers.get(&id).cloned()
    }

    fn add(&mut self, mut customer: Customer) -> u32 {
        let id = self.next_id;
        customer.id = id;
        self.customers.insert(id, customer);
        self.next_id += 1;
        id
    }

    fn update(&mut self, id: u32, customer: Customer) -> bool {
        if self.customers.contains_key(&id) {
            self.customers.insert(id, customer);
            true
        } else {
            false
        }
    }

    fn delete(&mut self, id: u32) -> bool {
        self.customers.remove(&id).is_some()
    }
}
