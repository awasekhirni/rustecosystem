//! Task Management Application demonstrating SOLID principles in Rust
//!
//! This application showcases:
//! - S: Single Responsibility Principle
//! - O: Open/Closed Principle
//! - L: Liskov Substitution Principle
//! - I: Interface Segregation Principle
//! - D: Dependency Inversion Principle

mod notifications;
mod tasks;
mod users;

use notifications::notifier::ConsoleNotifier;
use tasks::repository::InMemoryTaskRepository;
use tasks::service::TaskService;
use users::repository::InMemoryUserRepository;

fn main() {
    println!("Task Management Application");

    // Initialize dependencies
    let task_repo = InMemoryTaskRepository::new();
    let user_repo = InMemoryUserRepository::new();
    let notifier = ConsoleNotifier;

    // Create service with dependencies injected (Dependency Inversion)
    let mut task_service = TaskService::new(task_repo, user_repo, notifier);

    // Example usage - Note the Some() wrapper around the user ID
    match task_service.create_task(
        "Lets Complete RUST and Move to Dlang, Crystal and Haskell Programming Soon!",
        Some(1),
    ) {
        Ok(task) => println!("Created task: {:?}", task),
        Err(e) => println!("Error creating task: {}", e),
    }

    match task_service.complete_task(1, 1) {
        Ok(_) => println!("Task completed successfully"),
        Err(e) => println!("Error completing task: {}", e),
    }
}
