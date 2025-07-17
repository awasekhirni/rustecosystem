//! Playlist management module
//!
//! Handles creation, modification, and organization of playlists.
//! Follows SOLID principles with clear separation of concerns.

mod entities;
mod impls;
mod traits;

pub use entities::*;
pub use impls::*;
pub use traits::*;
