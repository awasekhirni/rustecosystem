use crate::models::WeatherData;

/// WeatherDisplay trait defines display capabilities
/// Demonstrates Interface Segregation Principle (ISP) by keeping interfaces small and focused
pub trait WeatherDisplay {
    fn display_current(&self, weather: &WeatherData);
    fn display_forecast(&self, forecast: &[WeatherData]);
}

/// Console implementation of WeatherDisplay
pub struct ConsoleWeatherDisplay;

impl WeatherDisplay for ConsoleWeatherDisplay {
    fn display_current(&self, weather: &WeatherData) {
        println!("Current Weather:");
        println!("Temperature: {:.1}°C", weather.temperature);
        println!("Humidity: {:.1}%", weather.humidity);
        println!("Pressure: {:.1} hPa", weather.pressure);
        println!("Wind Speed: {:.1} m/s", weather.wind_speed);
        println!("Conditions: {}", weather.conditions);
    }

    fn display_forecast(&self, forecast: &[WeatherData]) {
        println!("\nWeather Forecast ({} days):", forecast.len());
        for (i, weather) in forecast.iter().enumerate() {
            println!("\nDay {}:", i + 1);
            println!("  Temperature: {:.1}°C", weather.temperature);
            println!("  Conditions: {}", weather.conditions);
        }
    }
}
