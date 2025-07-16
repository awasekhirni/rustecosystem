mod customers;
mod notifications;

use customers::repository::CustomerRepository;
use customers::service::CustomerService;
use notifications::notifier::{CompositeNotifier, EmailNotifier, SmsNotifier};

fn main() {
    // Initialize dependencies
    let repository = CustomerRepository::new();
    let email_notifier = EmailNotifier;
    let sms_notifier = SmsNotifier;

    // Composite notifier (can easily add more notifiers without changing client code)
    let composite_notifier =
        CompositeNotifier::new(vec![Box::new(email_notifier), Box::new(sms_notifier)]);

    // Create customer service with dependencies injected
    let customer_service = CustomerService::new(repository, composite_notifier);

    // Use the service
    let customer = customer_service.register_customer("John Doe", "john@example.com", "+123456789");
    println!("Registered customer: {:?}", customer);

    let customers = customer_service.get_all_customers();
    println!("All customers: {:?}", customers);
}
