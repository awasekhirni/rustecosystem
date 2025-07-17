//! Audio player module
//!
//! Handles all audio playback functionality including play, pause, skip, etc.
//! Implements the SOLID principles through trait-based design.

//! Audio player module

mod entities;
mod impls;
mod traits;

pub use entities::PlayerState;
pub use impls::Player;
pub use traits::PlayerControls; // Explicitly re-export the trait
