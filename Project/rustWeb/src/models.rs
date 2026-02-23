// ============================================================
// models.rs — Data structures for our app
// ============================================================

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// ------------------------------------------------------------
// Todo
// ------------------------------------------------------------

/// A single todo item stored in memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

/// Payload for creating a new Todo (from JSON body or form)
#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
}

/// Payload for updating a Todo
#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

// ------------------------------------------------------------
// User
// ------------------------------------------------------------

/// A user account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Member,
    Guest,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserRole::Admin  => write!(f, "admin"),
            UserRole::Member => write!(f, "member"),
            UserRole::Guest  => write!(f, "guest"),
        }
    }
}

/// Payload for creating a new User
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub role: Option<UserRole>,
}

// ------------------------------------------------------------
// API Response envelope
// ------------------------------------------------------------

/// Standard JSON response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub count: Option<usize>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message: None,
            count: None,
        }
    }

    pub fn ok_list(data: T, count: usize) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message: None,
            count: Some(count),
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        ApiResponse {
            success: false,
            data: None,
            message: Some(msg.into()),
            count: None,
        }
    }
}
