// Main application demonstrating SOLID principles in Rust
mod display;
mod error;
mod models;
mod services;

use crate::display::weather_display::ConsoleWeatherDisplay;
use crate::display::weather_display::WeatherDisplay;
use crate::models::location::Location;
use crate::services::api_client::MockApiClient;
use crate::services::weather_service::WeatherService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Dependency Injection demonstrating Dependency Inversion Principle (DIP)
    let api_client = MockApiClient;
    let weather_service = WeatherService::new(api_client);
    let display = ConsoleWeatherDisplay;

    // Create a location
    let location = Location::new("London", "UK", 51.5074, -0.1278);

    // Get current weather (Single Responsibility)
    let current_weather = weather_service.get_current_weather(&location).await?;
    display.display_current(&current_weather);

    // Get forecast (Open/Closed Principle)
    let forecast = weather_service.get_forecast(&location, 3).await?;
    display.display_forecast(&forecast);

    Ok(())
}
