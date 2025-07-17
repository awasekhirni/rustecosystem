//! User-related data structures

/// User preferences
#[derive(Debug, Clone)]
pub struct UserPreferences {
    pub theme: String,
    pub autoplay: bool,
    pub explicit_content: bool,
    pub streaming_quality: String,
}
