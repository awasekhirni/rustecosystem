//! Recommendation-related data structures

/// User listening history item
#[derive(Debug, Clone)]
pub struct ListeningHistory {
    pub user_id: String,
    pub track_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub listened_seconds: f64,
}

/// Recommendation data structure
#[derive(Debug, Clone)]
pub struct Recommendation {
    pub track_id: String,
    pub reason: String,
    pub score: f32,
}
