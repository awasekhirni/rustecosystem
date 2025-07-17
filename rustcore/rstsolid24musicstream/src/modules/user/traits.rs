//! User management traits

use std::error::Error;

/// Interface for user authentication
pub trait UserAuthenticator {
    /// Authenticate a user
    fn authenticate(&self, username: &str, password: &str) -> Result<String, Box<dyn Error>>;

    /// Validate a session token
    fn validate_token(&self, token: &str) -> Result<bool, Box<dyn Error>>;
}

/// Interface for user profile management
pub trait UserProfileManager {
    /// Create a new user profile
    fn create_profile(&self, user: NewUser) -> Result<UserProfile, Box<dyn Error>>;

    /// Update user profile
    fn update_profile(&self, user: UserUpdate) -> Result<UserProfile, Box<dyn Error>>;

    /// Get user profile
    fn get_profile(&self, user_id: &str) -> Option<UserProfile>;
}

/// User data structures
#[derive(Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct UserUpdate {
    pub user_id: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}
