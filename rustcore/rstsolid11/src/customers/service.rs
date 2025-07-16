use crate::customers::customer::Customer;
use crate::customers::repository::CustomerRepositoryTrait;
use crate::notifications::notifier::Notifier;

// Service that follows Single Responsibility Principle (S in SOLID)
pub struct CustomerService<R, N>
where
    R: CustomerRepositoryTrait,
    N: Notifier,
{
    repository: R,
    notifier: N,
}

impl<R, N> CustomerService<R, N>
where
    R: CustomerRepositoryTrait,
    N: Notifier,
{
    pub fn new(repository: R, notifier: N) -> Self {
        CustomerService {
            repository,
            notifier,
        }
    }

    pub fn register_customer(&self, name: &str, email: &str, phone: &str) -> Customer {
        let customer = Customer::new(0, name, email, phone);
        let saved_customer = self.repository.save(customer);

        // Notify about new customer (Open/Closed Principle - O in SOLID)
        self.notifier
            .notify(&format!("New customer registered: {}", saved_customer));

        saved_customer
    }

    pub fn get_all_customers(&self) -> Vec<Customer> {
        self.repository.find_all()
    }
}
