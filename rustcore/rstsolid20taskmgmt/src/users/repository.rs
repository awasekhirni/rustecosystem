//! User repository trait and implementations

use crate::users::user::User;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Error type for user repository operations
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("User not found")]
    NotFound,
    #[error("User already exists")]
    AlreadyExists,
}

/// Minimal interface needed by task service (Interface Segregation)
pub trait UserRepository: Send + Sync {
    fn user_exists(&self, id: u64) -> bool;
    fn get_by_id(&self, id: u64) -> Result<User, RepositoryError>;
}

/// In-memory implementation of UserRepository
#[derive(Debug, Clone)]
pub struct InMemoryUserRepository {
    users: Arc<Mutex<HashMap<u64, User>>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(1, User::new(1, "Admin", "admin@example.com"));
        users.insert(2, User::new(2, "Developer", "dev@example.com"));

        Self {
            users: Arc::new(Mutex::new(users)),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn user_exists(&self, id: u64) -> bool {
        let users = self.users.lock().unwrap();
        users.contains_key(&id)
    }

    fn get_by_id(&self, id: u64) -> Result<User, RepositoryError> {
        let users = self.users.lock().unwrap();
        users.get(&id).cloned().ok_or(RepositoryError::NotFound)
    }
}
