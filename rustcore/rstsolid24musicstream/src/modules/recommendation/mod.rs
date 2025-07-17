//! Recommendation module
//!
//! Provides personalized music recommendations based on user behavior.
//! Follows SOLID principles with strategy pattern for different recommendation algorithms.

//! Recommendation module

mod entities;
mod impls;
mod traits;

pub use entities::Recommendation; // Correct export name
pub use impls::{
    CollaborativeFilteringRecommendation, ContentBasedRecommendation, PopularityRecommendation,
    RecommendationEngine,
};
pub use traits::RecommendationStrategy;
