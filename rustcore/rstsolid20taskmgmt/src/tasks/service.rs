//! Task service layer implementing business logic
//!
//! Demonstrates:
//! - Single Responsibility: Handles task-related business logic only
//! - Dependency Inversion: Depends on abstract traits, not concrete implementations
//! - Interface Segregation: Uses only needed methods from dependencies

use crate::notifications::notifier::Notifier;
use crate::tasks::{
    repository::TaskRepository,
    task::{Task, TaskStatus},
};
use crate::users::repository::UserRepository;
use thiserror::Error;

/// Error type for task service operations
#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] crate::tasks::repository::RepositoryError),
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid task state transition")]
    InvalidStateTransition,
}

/// Task service handling business logic
pub struct TaskService<T, U, N> {
    task_repository: T,
    user_repository: U,
    notifier: N,
}

impl<T, U, N> TaskService<T, U, N>
where
    T: TaskRepository,
    U: UserRepository,
    N: Notifier,
{
    /// Creates a new TaskService with injected dependencies
    pub fn new(task_repository: T, user_repository: U, notifier: N) -> Self {
        Self {
            task_repository,
            user_repository,
            notifier,
        }
    }

    /// Creates a new task and assigns it to a user if user_id is provided
    pub fn create_task(&mut self, title: &str, user_id: Option<u64>) -> Result<Task, ServiceError> {
        // Validate user exists if assigned
        if let Some(user_id) = user_id {
            if !self.user_repository.user_exists(user_id) {
                return Err(ServiceError::UserNotFound);
            }
        }

        // Generate ID (in a real app, this would come from the repository)
        let next_id = self
            .task_repository
            .get_all()
            .map(|tasks| tasks.len() as u64 + 1)
            .unwrap_or(1);

        let task = Task::new(next_id, title, user_id);
        let task = self.task_repository.create(task)?;

        // Notify user if assigned
        if let Some(user_id) = user_id {
            let user = self
                .user_repository
                .get_by_id(user_id)
                .map_err(|_| ServiceError::UserNotFound)?;
            self.notifier
                .send_notification(&user.email, &format!("New task assigned: {}", task.title));
        }

        Ok(task)
    }

    /// Completes a task if it's in a valid state
    pub fn complete_task(&mut self, task_id: u64, user_id: u64) -> Result<(), ServiceError> {
        let mut task = self.task_repository.get_by_id(task_id)?;

        // Check if task is assigned to this user
        if task.assigned_user_id != Some(user_id) {
            return Err(ServiceError::UserNotFound);
        }

        // Validate state transition
        match task.status {
            TaskStatus::Pending | TaskStatus::InProgress => {
                task.complete();
                self.task_repository.update(task)?;
                Ok(())
            }
            _ => Err(ServiceError::InvalidStateTransition),
        }
    }
}
