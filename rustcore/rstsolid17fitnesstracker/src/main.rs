//! Fitness Tracker Application demonstrating SOLID principles in Rust
//!
//! This application tracks fitness activities for users and generates reports.
//! Each module demonstrates one or more SOLID principles.

//! Fitness Tracker Application demonstrating SOLID principles in Rust

//! Fitness Tracker Application demonstrating SOLID principles in Rust

mod activities;
mod reports;
mod tracker;
mod user;

use activities::{Activity, Running, Swimming};
use chrono::Local;
use reports::ReportGenerator;
use tracker::FitnessTracker;
use user::User;

fn main() {
    // Create a user (Single Responsibility Principle)
    let user = User::new("John Doe", 30, 75.5);

    // Create fitness tracker (Dependency Inversion Principle)
    let mut tracker = FitnessTracker::new(user);

    // Add activities (Open/Closed Principle)
    let run = Running::new(Local::now().naive_local(), 45, 5.2, 140);
    tracker.add_activity(Box::new(run));

    let swim = Swimming::new(Local::now().naive_local(), 30, 1.5, 120, "freestyle");
    tracker.add_activity(Box::new(swim));

    // Generate report (Interface Segregation Principle)
    let report = tracker.generate_report();
    println!("{}", report);

    // Display user stats (Liskov Substitution Principle)
    tracker.display_user_stats();
}
