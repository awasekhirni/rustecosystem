use crate::customers::customer::Customer;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

// Repository trait for Dependency Inversion (D in SOLID)
pub trait CustomerRepositoryTrait: Send + Sync {
    fn save(&self, customer: Customer) -> Customer;
    fn find_all(&self) -> Vec<Customer>;
}

// Concrete implementation
pub struct CustomerRepository {
    customers: Mutex<Vec<Customer>>,
    next_id: AtomicU32,
}

impl CustomerRepository {
    pub fn new() -> Self {
        CustomerRepository {
            customers: Mutex::new(Vec::new()),
            next_id: AtomicU32::new(1),
        }
    }
}

impl CustomerRepositoryTrait for CustomerRepository {
    fn save(&self, customer: Customer) -> Customer {
        let mut customers = self.customers.lock().unwrap();
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let customer = Customer::new(id, &customer.name, &customer.email, &customer.phone);
        customers.push(customer.clone());
        customer
    }

    fn find_all(&self) -> Vec<Customer> {
        let customers = self.customers.lock().unwrap();
        customers.clone()
    }
}
