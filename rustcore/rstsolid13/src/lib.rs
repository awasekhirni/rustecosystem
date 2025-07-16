//! Library Management System demonstrating SOLID principles in Rust
//!
//! # Modules
//! - `book`: Contains Book entity and related traits (Single Responsibility)
//! - `inventory`: Manages book inventory (Open/Closed, Interface Segregation)
//! - `report`: Handles reporting (Dependency Inversion)
//! - `user`: Manages users (Liskov Substitution)

pub mod book;
pub mod inventory;
pub mod report;
pub mod user;
