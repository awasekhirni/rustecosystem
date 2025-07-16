pub mod notifiable;
pub mod repository;

// Re-export the traits at the crate level
pub use notifiable::Notifiable;
pub use repository::Repository;
