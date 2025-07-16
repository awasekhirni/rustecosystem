use super::Activity;
use chrono::NaiveDateTime;
use std::fmt;

/// Swimming activity implementation
/// Demonstrates Liskov Substitution Principle - can be used anywhere Activity is expected
#[derive(Debug)]
pub struct Swimming {
    start_time: NaiveDateTime,
    duration_minutes: u32,
    laps: f32,
    average_heart_rate: u32,
    stroke_type: String,
}

impl Swimming {
    /// Creates a new Swimming activity
    pub fn new(
        start_time: NaiveDateTime,
        duration_minutes: u32,
        laps: f32,
        average_heart_rate: u32,
        stroke_type: &str,
    ) -> Self {
        Swimming {
            start_time,
            duration_minutes,
            laps,
            average_heart_rate,
            stroke_type: stroke_type.to_string(),
        }
    }

    /// Returns the number of laps
    pub fn laps(&self) -> f32 {
        self.laps
    }

    /// Returns the stroke type
    pub fn stroke_type(&self) -> &str {
        &self.stroke_type
    }
}

impl Activity for Swimming {
    fn start_time(&self) -> NaiveDateTime {
        self.start_time
    }

    fn duration_minutes(&self) -> u32 {
        self.duration_minutes
    }

    fn calories_burned(&self) -> f32 {
        // Simple calculation for demonstration
        (self.duration_minutes as f32 * self.average_heart_rate as f32) / 15.0
    }

    fn summary(&self) -> String {
        format!(
            "Swam {} laps ({}) in {} minutes with average HR {} bpm",
            self.laps, self.stroke_type, self.duration_minutes, self.average_heart_rate
        )
    }
}

impl fmt::Display for Swimming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Swimming Activity: {}", self.summary())
    }
}
