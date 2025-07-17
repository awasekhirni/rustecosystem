use super::traits::*;
use std::error::Error;

// Import types through the public re-exports
use crate::modules::library::{Album, Artist, Track};

/// Concrete music library implementation
#[derive(Debug)]
pub struct MusicLibraryImpl {
    tracks: Vec<Track>,
    albums: Vec<Album>,
    artists: Vec<Artist>,
}

impl MusicLibraryImpl {
    /// Create a new music library
    pub fn new() -> Self {
        MusicLibraryImpl {
            tracks: Vec::new(),
            albums: Vec::new(),
            artists: Vec::new(),
        }
    }
}

impl MusicCatalog for MusicLibraryImpl {
    fn add_track(&mut self, track: Track) -> Result<(), Box<dyn Error>> {
        self.tracks.push(track);
        Ok(())
    }

    fn add_album(&mut self, album: Album) -> Result<(), Box<dyn Error>> {
        self.albums.push(album);
        Ok(())
    }

    fn add_artist(&mut self, artist: Artist) -> Result<(), Box<dyn Error>> {
        self.artists.push(artist);
        Ok(())
    }
}

impl MusicQuery for MusicLibraryImpl {
    fn search_tracks(&self, _query: &str) -> Vec<Track> {
        Vec::new()
    }

    fn get_artist_tracks(&self, _artist_id: &str) -> Vec<Track> {
        Vec::new()
    }

    fn get_album_tracks(&self, _album_id: &str) -> Vec<Track> {
        Vec::new()
    }
}
