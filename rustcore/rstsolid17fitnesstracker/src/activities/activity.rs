use chrono::NaiveDateTime;
use std::fmt;

/// Activity trait defining common behavior for all fitness activities
/// Demonstrates Open/Closed Principle - new activities can be added without modifying existing code
pub trait Activity: fmt::Display {
    /// Returns the start time of the activity
    fn start_time(&self) -> NaiveDateTime;

    /// Returns the duration of the activity in minutes
    fn duration_minutes(&self) -> u32;

    /// Returns the calories burned during the activity
    fn calories_burned(&self) -> f32;

    /// Returns a summary of the activity
    fn summary(&self) -> String;
}
