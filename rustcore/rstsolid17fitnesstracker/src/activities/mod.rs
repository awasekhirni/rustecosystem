//! Activities module handles different types of fitness activities

pub mod activity;
pub mod running;
pub mod swimming;

// Re-export the public interface
pub use self::activity::Activity;
pub use self::running::Running;
pub use self::swimming::Swimming;
