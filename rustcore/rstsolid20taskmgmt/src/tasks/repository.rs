//! Task repository trait and implementations
//!
//! Demonstrates:
//! - Dependency Inversion: High-level modules depend on the TaskRepository trait
//! - Open/Closed: New repository types can be added without changing existing code
//! - Liskov Substitution: Any implementation can be substituted for another

use crate::tasks::task::{Task, TaskStatus};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;

/// Error type for task repository operations
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Task not found")]
    NotFound,
    #[error("Task already exists")]
    AlreadyExists,
    #[error("Database error")]
    DatabaseError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Trait defining task repository operations (Interface Segregation)
pub trait TaskRepository: Send + Sync {
    fn create(&mut self, task: Task) -> Result<Task, RepositoryError>;
    fn get_by_id(&self, id: u64) -> Result<Task, RepositoryError>;
    fn get_all(&self) -> Result<Vec<Task>, RepositoryError>;
    fn update(&mut self, task: Task) -> Result<Task, RepositoryError>;
    fn delete(&mut self, id: u64) -> Result<(), RepositoryError>;
    fn get_by_status(&self, status: TaskStatus) -> Result<Vec<Task>, RepositoryError>;
}

/// In-memory implementation of TaskRepository
#[derive(Debug, Clone)]
pub struct InMemoryTaskRepository {
    tasks: Arc<Mutex<HashMap<u64, Task>>>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn create(&mut self, task: Task) -> Result<Task, RepositoryError> {
        let mut tasks = self.tasks.lock().unwrap();
        if tasks.contains_key(&task.id) {
            return Err(RepositoryError::AlreadyExists);
        }
        tasks.insert(task.id, task.clone());
        Ok(task)
    }

    fn get_by_id(&self, id: u64) -> Result<Task, RepositoryError> {
        let tasks = self.tasks.lock().unwrap();
        tasks.get(&id).cloned().ok_or(RepositoryError::NotFound)
    }

    fn get_all(&self) -> Result<Vec<Task>, RepositoryError> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks.values().cloned().collect())
    }

    fn update(&mut self, task: Task) -> Result<Task, RepositoryError> {
        let mut tasks = self.tasks.lock().unwrap();
        if !tasks.contains_key(&task.id) {
            return Err(RepositoryError::NotFound);
        }
        tasks.insert(task.id, task.clone());
        Ok(task)
    }

    fn delete(&mut self, id: u64) -> Result<(), RepositoryError> {
        let mut tasks = self.tasks.lock().unwrap();
        tasks
            .remove(&id)
            .map(|_| ())
            .ok_or(RepositoryError::NotFound)
    }

    fn get_by_status(&self, status: TaskStatus) -> Result<Vec<Task>, RepositoryError> {
        let tasks = self.tasks.lock().unwrap();
        Ok(tasks
            .values()
            .filter(|t| t.status == status)
            .cloned()
            .collect())
    }
}
