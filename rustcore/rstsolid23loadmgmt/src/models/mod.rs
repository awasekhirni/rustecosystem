//! Domain models for the Loan Management System

pub mod customer;
pub mod loan;

pub use customer::Customer;
pub use loan::{Loan, LoanApplication, LoanError, LoanStatus}; // Added LoanError here
