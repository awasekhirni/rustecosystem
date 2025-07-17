//! Main modules module that re-exports all submodules

//! Main modules module

pub mod library;
pub mod player;
pub mod playlist;
pub mod recommendation;
pub mod user;

// Create a prelude module
pub mod prelude {
    pub use super::library::MusicLibraryImpl as MusicLibrary;
    pub use super::player::{Player, PlayerControls};
    pub use super::playlist::{Playlist, PlaylistManagerImpl as PlaylistManager};
    pub use super::recommendation::RecommendationEngine;
    pub use super::user::UserManagerImpl as UserManager;
}
