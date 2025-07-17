//! Music library traits

use std::error::Error;

/// Interface for music catalog management
pub trait MusicCatalog {
    /// Add a new track to the library
    fn add_track(&mut self, track: Track) -> Result<(), Box<dyn Error>>;

    /// Add a new album to the library
    fn add_album(&mut self, album: Album) -> Result<(), Box<dyn Error>>;

    /// Add a new artist to the library
    fn add_artist(&mut self, artist: Artist) -> Result<(), Box<dyn Error>>;
}

/// Interface for music catalog queries
pub trait MusicQuery {
    /// Search tracks by title
    fn search_tracks(&self, query: &str) -> Vec<Track>;

    /// Get tracks by artist
    fn get_artist_tracks(&self, artist_id: &str) -> Vec<Track>;

    /// Get album tracks
    fn get_album_tracks(&self, album_id: &str) -> Vec<Track>;
}

/// Music library data structures
#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist_id: String,
    pub album_id: String,
    pub duration: f64,
    pub genre: String,
    pub release_date: chrono::NaiveDate,
    pub audio_url: String,
}

#[derive(Debug, Clone)]
pub struct Album {
    pub id: String,
    pub title: String,
    pub artist_id: String,
    pub release_date: chrono::NaiveDate,
    pub cover_art_url: String,
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub bio: Option<String>,
    pub image_url: Option<String>,
}
