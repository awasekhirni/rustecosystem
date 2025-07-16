use std::fmt;

/// Represents a hotel room
/// Demonstrates Single Responsibility Principle - only handles room data
#[derive(Debug, Clone)]
pub struct Room {
    pub id: u32,
    pub category: String,
    pub price_per_night: f64,
}

impl Room {
    /// Creates a new Room instance
    pub fn new(id: u32, category: &str, price_per_night: f64) -> Self {
        Room {
            id,
            category: category.to_string(),
            price_per_night,
        }
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Room {} - {} (${}/night)",
            self.id, self.category, self.price_per_night
        )
    }
}
