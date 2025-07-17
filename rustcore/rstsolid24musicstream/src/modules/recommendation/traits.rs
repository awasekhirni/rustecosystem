//! Recommendation engine traits

use super::entities::Recommendation;
use std::error::Error; // Import from local module

/// Interface for recommendation strategies
pub trait RecommendationStrategy {
    /// Generate recommendations for a user
    fn recommend(&self, user_id: &str) -> Result<Vec<Recommendation>, Box<dyn Error>>;
}
