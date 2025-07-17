//! Music library module
//!
//! Manages the music catalog, including tracks, albums, and artists.
//! Follows SOLID principles with clear separation between interfaces and implementations.

//! Music library module

mod entities;
mod impls;
mod traits;

pub use entities::{Album, Artist, Track};
pub use impls::MusicLibraryImpl;
pub use traits::*; // Explicitly re-export entities
