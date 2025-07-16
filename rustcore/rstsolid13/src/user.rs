//! User module demonstrating Liskov Substitution Principle (LSP)
//!
//! Subtypes (different user types) can be substituted for their base type

/// Base user trait
pub trait User {
    fn borrow_book(&mut self, isbn: &str) -> Result<(), String>;
    fn return_book(&mut self, isbn: &str) -> Result<(), String>;
    fn get_username(&self) -> &str;
    fn list_borrowed_books(&self) -> &[String];
}

/// Regular library user
pub struct RegularUser {
    username: String,
    borrowed_books: Vec<String>,
}

impl RegularUser {
    pub fn new(username: &str) -> Self {
        RegularUser {
            username: username.to_string(),
            borrowed_books: Vec::new(),
        }
    }
}

impl User for RegularUser {
    fn borrow_book(&mut self, isbn: &str) -> Result<(), String> {
        if self.borrowed_books.len() >= 5 {
            return Err("Regular users can borrow at most 5 books".to_string());
        }
        self.borrowed_books.push(isbn.to_string());
        Ok(())
    }

    fn return_book(&mut self, isbn: &str) -> Result<(), String> {
        let pos = self
            .borrowed_books
            .iter()
            .position(|b| b == isbn)
            .ok_or_else(|| format!("Book with ISBN {} not borrowed", isbn))?;
        self.borrowed_books.remove(pos);
        Ok(())
    }

    fn get_username(&self) -> &str {
        &self.username
    }

    fn list_borrowed_books(&self) -> &[String] {
        &self.borrowed_books
    }
}

/// Premium user with extended privileges
pub struct PremiumUser {
    username: String,
    borrowed_books: Vec<String>,
}

impl PremiumUser {
    pub fn new(username: &str) -> Self {
        PremiumUser {
            username: username.to_string(),
            borrowed_books: Vec::new(),
        }
    }
}

impl User for PremiumUser {
    fn borrow_book(&mut self, isbn: &str) -> Result<(), String> {
        if self.borrowed_books.len() >= 10 {
            return Err("Premium users can borrow at most 10 books".to_string());
        }
        self.borrowed_books.push(isbn.to_string());
        Ok(())
    }

    fn return_book(&mut self, isbn: &str) -> Result<(), String> {
        let pos = self
            .borrowed_books
            .iter()
            .position(|b| b == isbn)
            .ok_or_else(|| format!("Book with ISBN {} not borrowed", isbn))?;
        self.borrowed_books.remove(pos);
        Ok(())
    }

    fn get_username(&self) -> &str {
        &self.username
    }

    fn list_borrowed_books(&self) -> &[String] {
        &self.borrowed_books
    }
}

/// Librarian with admin privileges
pub struct Librarian {
    username: String,
    borrowed_books: Vec<String>,
}

impl Librarian {
    pub fn new(username: &str) -> Self {
        Librarian {
            username: username.to_string(),
            borrowed_books: Vec::new(),
        }
    }

    pub fn add_new_book(
        &self,
        inventory: &mut dyn crate::inventory::Inventory,
        book: crate::book::Book,
    ) -> Result<(), String> {
        inventory.add_book(book)
    }
}

impl User for Librarian {
    fn borrow_book(&mut self, isbn: &str) -> Result<(), String> {
        self.borrowed_books.push(isbn.to_string());
        Ok(())
    }

    fn return_book(&mut self, isbn: &str) -> Result<(), String> {
        let pos = self
            .borrowed_books
            .iter()
            .position(|b| b == isbn)
            .ok_or_else(|| format!("Book with ISBN {} not borrowed", isbn))?;
        self.borrowed_books.remove(pos);
        Ok(())
    }

    fn get_username(&self) -> &str {
        &self.username
    }

    fn list_borrowed_books(&self) -> &[String] {
        &self.borrowed_books
    }
}
