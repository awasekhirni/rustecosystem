use crate::error::WeatherError;
use crate::models::{Location, WeatherData};
use crate::services::ApiClient;

/// WeatherService handles business logic for weather operations
/// Demonstrates Open/Closed Principle (OCP) by being open for extension but closed for modification
pub struct WeatherService<C: ApiClient> {
    api_client: C,
}

impl<C: ApiClient> WeatherService<C> {
    /// Creates a new WeatherService with the given ApiClient
    pub fn new(api_client: C) -> Self {
        WeatherService { api_client }
    }

    /// Gets current weather for a location
    pub async fn get_current_weather(
        &self,
        location: &Location,
    ) -> Result<WeatherData, WeatherError> {
        self.api_client.fetch_weather(location).await
    }

    /// Gets weather forecast for a location (extended functionality without modifying existing code)
    pub async fn get_forecast(
        &self,
        location: &Location,
        days: u8,
    ) -> Result<Vec<WeatherData>, WeatherError> {
        let mut forecast = Vec::new();

        for _ in 0..days {
            let weather = self.api_client.fetch_weather(location).await?;
            forecast.push(weather);
        }

        Ok(forecast)
    }
}
