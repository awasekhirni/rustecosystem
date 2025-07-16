//! Library Management System - Main Application
//!
//! Demonstrates SOLID principles in action:
//! - Single Responsibility: Each module has a single responsibility
//! - Open/Closed: Inventory and reports can be extended without modification
//! - Liskov Substitution: Different user types can be substituted
//! - Interface Segregation: Small, focused traits
//! - Dependency Inversion: High-level modules depend on abstractions

mod book;
mod inventory;
mod report;
mod user;

use crate::{
    book::{Book, BookDisplay, BookValidator},
    inventory::{BasicInventory, EnhancedInventory, Inventory},
    report::{BasicReportGenerator, DetailedReportGenerator, ReportGenerator},
    user::{Librarian, PremiumUser, RegularUser, User},
};

fn main() {
    // Create some books
    let book1 = Book {
        isbn: "978-3-16-148410-0".to_string(),
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        year: 2022,
    };

    let book2 = Book {
        isbn: "978-0132350884".to_string(),
        title: "Clean Code".to_string(),
        author: "Robert C. Martin".to_string(),
        year: 2008,
    };

    // Validate a book (SRP and ISP)
    if let Err(e) = book1.validate() {
        println!("Validation error: {}", e);
    }

    // Create inventory (OCP - can choose Basic or Enhanced without changing other code)
    let mut inventory = EnhancedInventory::new();

    // Add books to inventory
    if let Err(e) = inventory.add_book(book1.clone()) {
        println!("Error adding book: {}", e);
    }

    if let Err(e) = inventory.add_book(book2.clone()) {
        println!("Error adding book: {}", e);
    }

    // Generate reports (DIP - depends on Inventory trait)
    let basic_report = BasicReportGenerator;
    println!(
        "\nBasic Report:\n{}",
        basic_report.generate_report(&inventory)
    );

    let detailed_report = DetailedReportGenerator;
    println!(
        "\nDetailed Report:\n{}",
        detailed_report.generate_report(&inventory)
    );

    // Demonstrate LSP with different user types
    // Create mutable users
    let mut regular_user = RegularUser::new("alice");
    let mut premium_user = PremiumUser::new("bob");
    let mut librarian = Librarian::new("carol");

    // Call with mutable references
    borrow_book_for_user(&mut regular_user, &book1.isbn);
    borrow_book_for_user(&mut premium_user, &book1.isbn);
    borrow_book_for_user(&mut librarian, &book1.isbn);

    // Librarian-specific functionality
    let new_book = Book {
        isbn: "978-0201633610".to_string(),
        title: "Design Patterns".to_string(),
        author: "Erich Gamma, Richard Helm, Ralph Johnson, John Vlissides".to_string(),
        year: 1994,
    };

    if let Err(e) = librarian.add_new_book(&mut inventory, new_book) {
        println!("Error adding book: {}", e);
    }

    // Display updated inventory
    println!("\nUpdated Inventory:");
    for book in inventory.list_books() {
        book.display();
    }
}

/// Function demonstrating Liskov Substitution Principle
/// It can work with any type implementing User trait
fn borrow_book_for_user(user: &mut dyn User, isbn: &str) {
    match user.borrow_book(isbn) {
        Ok(_) => println!(
            "{} successfully borrowed book {}",
            user.get_username(),
            isbn
        ),
        Err(e) => println!("Error for {}: {}", user.get_username(), e),
    }
}
