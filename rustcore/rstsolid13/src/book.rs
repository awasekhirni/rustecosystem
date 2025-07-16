//! Book module demonstrating Single Responsibility Principle (SRP)
//!
//! The Book module is only responsible for book-related functionality.
//! It defines the Book struct and traits for book operations.

/// Represents a book in the library
#[derive(Debug, Clone)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub year: u32,
}

/// Trait for book validation
/// Demonstrates Interface Segregation Principle (ISP)
/// by separating validation from other book operations
pub trait BookValidator {
    fn validate(&self) -> Result<(), String>;
}

impl BookValidator for Book {
    fn validate(&self) -> Result<(), String> {
        if self.isbn.is_empty() {
            return Err("ISBN cannot be empty".to_string());
        }
        if self.title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Trait for book display functionality
/// Another example of ISP - separating display from validation
pub trait BookDisplay {
    fn display(&self);
}

impl BookDisplay for Book {
    fn display(&self) {
        println!(
            "Book: {} by {} ({}, ISBN: {})",
            self.title, self.author, self.year, self.isbn
        );
    }
}
