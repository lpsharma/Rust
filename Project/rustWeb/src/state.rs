// ============================================================
// state.rs — Shared application state
// Passed to every handler via axum's State extractor
// ============================================================

use std::sync::Mutex;
use tera::Tera;
use crate::models::{Todo, User};

/// AppState is wrapped in Arc<> in main.rs so all handlers
/// can safely share it across async tasks.
pub struct AppState {
    /// Tera template engine (loaded once at startup)
    pub tera: Tera,

    /// In-memory todo storage (Mutex for safe concurrent access)
    pub todos: Mutex<Vec<Todo>>,

    /// In-memory user storage
    pub users: Mutex<Vec<User>>,
}
