/// Custom error types for the weather application
/// Demonstrates Single Responsibility Principle (SRP) by handling all error types in one place
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Location not found: {0}")]
    LocationNotFound(String),

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Unknown error")]
    Unknown,
}
