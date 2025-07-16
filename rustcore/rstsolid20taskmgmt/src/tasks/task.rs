//! Task entity and related types
//!
//! Demonstrates Single Responsibility Principle - this module only handles task data structure

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Task status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Archived,
}

/// Task entity representing a single task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub assigned_user_id: Option<u64>,
}

impl Task {
    /// Creates a new task with default status (Pending) and timestamps
    pub fn new(id: u64, title: &str, assigned_user_id: Option<u64>) -> Self {
        let now = Utc::now();
        Self {
            id,
            title: title.to_string(),
            description: None,
            status: TaskStatus::Pending,
            created_at: now,
            updated_at: now,
            assigned_user_id,
        }
    }

    /// Marks task as completed
    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.updated_at = Utc::now();
    }
}
