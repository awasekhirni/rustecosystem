//! User management module
//!
//! Handles user accounts, authentication, and profiles.
//! Follows SOLID principles with clear interfaces.

mod entities;
mod impls;
mod traits;

pub use entities::*;
pub use impls::*;
pub use traits::*;
