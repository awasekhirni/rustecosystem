mod models;
mod repositories;
mod services;
mod traits;

use models::customer::Customer;
use models::policy::{Policy, PolicyType};
use repositories::{customer_repository::CustomerRepository, policy_repository::PolicyRepository};
use services::{customer_service::CustomerService, policy_service::PolicyService};
use traits::{Notifiable, Repository};

struct EmailNotifier;

impl Notifiable for EmailNotifier {
    fn send_notification(&self, message: &str) -> bool {
        println!("[Email Notification]: {}", message);
        true
    }
}

fn main() {
    let customer_repo = CustomerRepository::new();
    let policy_repo = PolicyRepository::new();
    let notifier = EmailNotifier;

    let mut customer_service = CustomerService::new(customer_repo, notifier);
    let mut policy_service = PolicyService::new(policy_repo, EmailNotifier);

    let customer_id = customer_service
        .register_customer("John Doe".to_string(), "john.doe@example.com".to_string());
    println!("Registered customer with ID: {}", customer_id);

    let auto_policy_id = policy_service.create_policy(
        customer_id,
        PolicyType::Auto {
            vehicle_vin: "1HGBH41JXMN109186".to_string(),
        },
        1200.0,
    );
    println!("Created auto policy with ID: {}", auto_policy_id);

    let home_policy_id = policy_service.create_policy(
        customer_id,
        PolicyType::Home {
            address: "123 Main St".to_string(),
        },
        850.0,
    );
    println!("Created home policy with ID: {}", home_policy_id);

    if let Some(customer) = customer_service.get_customer(customer_id) {
        println!("\nCustomer Details:");
        println!("ID: {}", customer.id);
        println!("Name: {}", customer.name);
        println!("Email: {}", customer.email);
        println!("Policy IDs: {:?}", customer.policy_ids);
    }

    println!("\nAll Policies:");
    for policy in policy_service.get_all_policies() {
        println!(
            "ID: {}, Customer ID: {}, Premium: ${:.2}, Active: {}",
            policy.id, policy.customer_id, policy.premium, policy.active
        );
    }
}
