use super::Activity;
use chrono::NaiveDateTime;
use std::fmt;

/// Running activity implementation
#[derive(Debug)]
pub struct Running {
    start_time: NaiveDateTime,
    duration_minutes: u32,
    distance_km: f32,
    average_heart_rate: u32,
}

impl Running {
    /// Creates a new Running activity
    pub fn new(
        start_time: NaiveDateTime,
        duration_minutes: u32,
        distance_km: f32,
        average_heart_rate: u32,
    ) -> Self {
        Running {
            start_time,
            duration_minutes,
            distance_km,
            average_heart_rate,
        }
    }

    /// Returns the distance in kilometers
    pub fn distance_km(&self) -> f32 {
        self.distance_km
    }

    /// Returns the average heart rate
    pub fn average_heart_rate(&self) -> u32 {
        self.average_heart_rate
    }
}

impl Activity for Running {
    fn start_time(&self) -> NaiveDateTime {
        self.start_time
    }

    fn duration_minutes(&self) -> u32 {
        self.duration_minutes
    }

    fn calories_burned(&self) -> f32 {
        // Simple calculation for demonstration
        (self.duration_minutes as f32 * self.average_heart_rate as f32) / 20.0
    }

    fn summary(&self) -> String {
        format!(
            "Ran {} km in {} minutes with average HR {} bpm",
            self.distance_km, self.duration_minutes, self.average_heart_rate
        )
    }
}

impl fmt::Display for Running {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Running Activity: {}", self.summary())
    }
}
