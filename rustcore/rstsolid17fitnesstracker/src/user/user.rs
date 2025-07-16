use std::fmt;

/// User struct representing a fitness tracker user
/// Demonstrates Single Responsibility Principle - only handles user data
#[derive(Debug, Clone)]
pub struct User {
    name: String,
    age: u8,
    weight_kg: f32,
}

impl User {
    /// Creates a new User instance
    pub fn new(name: &str, age: u8, weight_kg: f32) -> Self {
        User {
            name: name.to_string(),
            age,
            weight_kg,
        }
    }

    /// Returns the user's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the user's age
    pub fn age(&self) -> u8 {
        self.age
    }

    /// Returns the user's weight in kg
    pub fn weight_kg(&self) -> f32 {
        self.weight_kg
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User: {}, Age: {}, Weight: {}kg",
            self.name, self.age, self.weight_kg
        )
    }
}
