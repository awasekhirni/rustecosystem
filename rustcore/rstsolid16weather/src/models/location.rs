use serde::{Deserialize, Serialize};

/// Location model representing a geographical location
/// Demonstrates SRP by only containing location-related data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub city: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    /// Creates a new Location instance
    pub fn new(city: &str, country: &str, latitude: f64, longitude: f64) -> Self {
        Location {
            city: city.to_string(),
            country: country.to_string(),
            latitude,
            longitude,
        }
    }
}
