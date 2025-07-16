use crate::error::WeatherError;
use crate::models::{Location, WeatherData};

/// ApiClient trait defining the interface for weather API clients
/// Demonstrates Dependency Inversion Principle (DIP) by depending on abstractions
#[async_trait::async_trait]
pub trait ApiClient: Send + Sync {
    async fn fetch_weather(&self, location: &Location) -> Result<WeatherData, WeatherError>;
}

/// Concrete implementation of ApiClient using a mock service
pub struct MockApiClient;

#[async_trait::async_trait]
impl ApiClient for MockApiClient {
    async fn fetch_weather(&self, location: &Location) -> Result<WeatherData, WeatherError> {
        // In a real application, this would make an HTTP request
        println!("Fetching weather for {}...", location.city);

        // Simulate API delay
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;

        Ok(WeatherData::new(
            22.5,                           // temperature
            65.0,                           // humidity
            1013.0,                         // pressure
            10.5,                           // wind_speed
            "Sunny",                        // conditions
            chrono::Utc::now().timestamp(), // timestamp
        ))
    }
}
