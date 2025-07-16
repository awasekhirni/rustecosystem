//! Inventory management demonstrating Open/Closed Principle (OCP)
//! and Dependency Inversion Principle (DIP)
//!
//! The inventory can be extended without modifying existing code (OCP)
//! and depends on abstractions (traits) rather than concrete implementations (DIP)

use crate::book::{Book, BookValidator};
use std::collections::HashMap;

/// Trait for inventory operations
/// This abstraction allows different inventory implementations
pub trait Inventory {
    fn add_book(&mut self, book: Book) -> Result<(), String>;
    fn remove_book(&mut self, isbn: &str) -> Result<(), String>;
    fn find_book(&self, isbn: &str) -> Option<&Book>;
    fn list_books(&self) -> Vec<&Book>;
}

/// Basic inventory implementation using HashMap
pub struct BasicInventory {
    books: HashMap<String, Book>,
}

impl BasicInventory {
    pub fn new() -> Self {
        BasicInventory {
            books: HashMap::new(),
        }
    }
}

impl Inventory for BasicInventory {
    fn add_book(&mut self, book: Book) -> Result<(), String> {
        // Validate book before adding (using dependency on BookValidator)
        book.validate()?;

        if self.books.contains_key(&book.isbn) {
            return Err(format!("Book with ISBN {} already exists", book.isbn));
        }

        self.books.insert(book.isbn.clone(), book);
        Ok(())
    }

    fn remove_book(&mut self, isbn: &str) -> Result<(), String> {
        self.books
            .remove(isbn)
            .map(|_| ())
            .ok_or_else(|| format!("Book with ISBN {} not found", isbn))
    }

    fn find_book(&self, isbn: &str) -> Option<&Book> {
        self.books.get(isbn)
    }

    fn list_books(&self) -> Vec<&Book> {
        self.books.values().collect()
    }
}

/// Enhanced inventory with additional features
/// Demonstrates Open/Closed Principle - extending without modifying existing code
pub struct EnhancedInventory {
    basic: BasicInventory,
    // Additional fields can be added here
}

impl EnhancedInventory {
    pub fn new() -> Self {
        EnhancedInventory {
            basic: BasicInventory::new(),
        }
    }

    // New functionality can be added without changing BasicInventory
    pub fn count_books(&self) -> usize {
        self.basic.books.len()
    }
}

impl Inventory for EnhancedInventory {
    fn add_book(&mut self, book: Book) -> Result<(), String> {
        // Can add additional checks or logging here
        println!("Adding book: {}", book.title);
        self.basic.add_book(book)
    }

    fn remove_book(&mut self, isbn: &str) -> Result<(), String> {
        println!("Removing book with ISBN: {}", isbn);
        self.basic.remove_book(isbn)
    }

    fn find_book(&self, isbn: &str) -> Option<&Book> {
        self.basic.find_book(isbn)
    }

    fn list_books(&self) -> Vec<&Book> {
        self.basic.list_books()
    }
}
