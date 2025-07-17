//! Playlist-related data structures

/// Playlist metadata
#[derive(Debug, Clone)]
pub struct PlaylistMetadata {
    pub description: Option<String>,
    pub cover_art_url: Option<String>,
    pub is_public: bool,
    pub last_modified: chrono::DateTime<chrono::Utc>,
}
