use serde::{Deserialize, Serialize};

/// WeatherData model containing weather information
/// Demonstrates SRP by only containing weather-related data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub wind_speed: f64,
    pub conditions: String,
    pub timestamp: i64,
}

impl WeatherData {
    /// Creates a new WeatherData instance
    pub fn new(
        temperature: f64,
        humidity: f64,
        pressure: f64,
        wind_speed: f64,
        conditions: &str,
        timestamp: i64,
    ) -> Self {
        WeatherData {
            temperature,
            humidity,
            pressure,
            wind_speed,
            conditions: conditions.to_string(),
            timestamp,
        }
    }
}
