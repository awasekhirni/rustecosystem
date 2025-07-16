//! Task management module implementing core business logic
//!
//! Demonstrates:
//! - Single Responsibility: Each struct has one clear purpose
//! - Open/Closed: Extendable through traits without modification
//! - Liskov Substitution: Repository implementations are interchangeable

pub mod repository;
pub mod service;
pub mod task;
