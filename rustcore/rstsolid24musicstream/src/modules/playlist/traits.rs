//! Playlist management traits

use std::error::Error;

/// Interface for playlist creation and management
pub trait PlaylistManager {
    /// Create a new playlist
    fn create_playlist(&self, name: &str, owner_id: &str) -> Result<Playlist, Box<dyn Error>>;

    /// Delete a playlist
    fn delete_playlist(&self, playlist_id: &str) -> Result<(), Box<dyn Error>>;

    /// Add track to playlist
    fn add_track(&self, playlist_id: &str, track_id: &str) -> Result<(), Box<dyn Error>>;

    /// Remove track from playlist
    fn remove_track(&self, playlist_id: &str, track_id: &str) -> Result<(), Box<dyn Error>>;
}

/// Interface for playlist retrieval
pub trait PlaylistRetriever {
    /// Get playlist by ID
    fn get_playlist(&self, playlist_id: &str) -> Option<Playlist>;

    /// Get all playlists for a user
    fn get_user_playlists(&self, user_id: &str) -> Vec<Playlist>;
}

/// Playlist data structure
#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub track_ids: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
