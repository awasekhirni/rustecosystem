//! Reporting module demonstrating Dependency Inversion Principle (DIP)
//!
//! High-level reporting depends on abstractions (traits) rather than concrete implementations

use crate::inventory::Inventory;

/// Trait for report generation
pub trait ReportGenerator {
    fn generate_report(&self, inventory: &dyn Inventory) -> String;
}

/// Basic report generator
pub struct BasicReportGenerator;

impl ReportGenerator for BasicReportGenerator {
    fn generate_report(&self, inventory: &dyn Inventory) -> String {
        let books = inventory.list_books();
        let mut report = String::from("Library Inventory Report\n");
        report.push_str("========================\n");

        for book in &books {
            report.push_str(&format!(
                "{} by {} ({})\n",
                book.title, book.author, book.year
            ));
        }

        report.push_str(&format!("\nTotal books: {}", books.len()));
        report
    }
}

/// Detailed report generator
pub struct DetailedReportGenerator;

impl ReportGenerator for DetailedReportGenerator {
    fn generate_report(&self, inventory: &dyn Inventory) -> String {
        let books = inventory.list_books();
        let mut report = String::from("Detailed Library Inventory Report\n");
        report.push_str("=================================\n");

        for book in &books {
            report.push_str(&format!(
                "Title: {}\nAuthor: {}\nYear: {}\nISBN: {}\n\n",
                book.title, book.author, book.year, book.isbn
            ));
        }

        report.push_str(&format!("Total books: {}", books.len()));
        report
    }
}
