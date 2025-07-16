use crate::models::customer::Customer;
use crate::repositories::customer_repository::CustomerRepository;
use crate::traits::{Notifiable, Repository};

pub struct CustomerService<T: Notifiable> {
    repository: CustomerRepository,
    notifier: T,
}

impl<T: Notifiable> CustomerService<T> {
    pub fn new(repository: CustomerRepository, notifier: T) -> Self {
        CustomerService {
            repository,
            notifier,
        }
    }

    pub fn register_customer(&mut self, name: String, email: String) -> u32 {
        let customer = Customer::new(0, name, email);
        let id = self.repository.add(customer);

        self.notifier.send_notification(&format!(
            "Welcome to our insurance system! Your customer ID is {}",
            id
        ));

        id
    }

    pub fn get_customer(&self, id: u32) -> Option<Customer> {
        self.repository.get_by_id(id)
    }

    pub fn get_all_customers(&self) -> Vec<Customer> {
        self.repository.get_all()
    }
}
