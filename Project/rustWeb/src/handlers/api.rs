// ============================================================
// handlers/api.rs — REST API handlers (return JSON)
// ============================================================

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::state::AppState;
use crate::models::{
    ApiResponse, CreateTodo, CreateUser, Todo, UpdateTodo, User, UserRole,
};

// ============================================================
// TODOS
// ============================================================

/// GET /api/todos — list all todos
pub async fn list_todos(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Vec<Todo>>> {
    let todos = state.todos.lock().unwrap().clone();
    let count = todos.len();
    Json(ApiResponse::ok_list(todos, count))
}

/// GET /api/todos/:id — get one todo
pub async fn get_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Todo>>, (StatusCode, Json<ApiResponse<Todo>>)> {
    let todos = state.todos.lock().unwrap();

    match todos.iter().find(|t| t.id == id) {
        Some(todo) => Ok(Json(ApiResponse::ok(todo.clone()))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(format!("Todo '{}' not found", id))),
        )),
    }
}

/// POST /api/todos — create a new todo
pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTodo>,
) -> (StatusCode, Json<ApiResponse<Todo>>) {
    // Validate
    if payload.title.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<Todo>::error("Title cannot be empty")),
        );
    }

    let todo = Todo {
        id:          Uuid::new_v4().to_string(),
        title:       payload.title.trim().to_string(),
        description: payload.description,
        completed:   false,
        created_at:  Utc::now(),
    };

    state.todos.lock().unwrap().push(todo.clone());
    tracing::info!("Created todo: {}", todo.id);

    (StatusCode::CREATED, Json(ApiResponse::ok(todo)))
}

/// PUT /api/todos/:id — update a todo
pub async fn update_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<ApiResponse<Todo>>, (StatusCode, Json<ApiResponse<Todo>>)> {
    let mut todos = state.todos.lock().unwrap();

    match todos.iter_mut().find(|t| t.id == id) {
        Some(todo) => {
            if let Some(title) = payload.title {
                todo.title = title;
            }
            if let Some(desc) = payload.description {
                todo.description = Some(desc);
            }
            if let Some(completed) = payload.completed {
                todo.completed = completed;
            }
            tracing::info!("Updated todo: {}", todo.id);
            Ok(Json(ApiResponse::ok(todo.clone())))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<Todo>::error(format!("Todo '{}' not found", id))),
        )),
    }
}

/// DELETE /api/todos/:id — delete a todo
pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut todos = state.todos.lock().unwrap();
    let before = todos.len();
    todos.retain(|t| t.id != id);

    if todos.len() < before {
        tracing::info!("Deleted todo: {}", id);
        Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: Some(format!("Todo '{}' deleted", id)),
            count: None,
        }))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(format!("Todo '{}' not found", id))),
        ))
    }
}

// ============================================================
// USERS
// ============================================================

/// GET /api/users — list all users
pub async fn list_users(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<Vec<User>>> {
    let users = state.users.lock().unwrap().clone();
    let count = users.len();
    Json(ApiResponse::ok_list(users, count))
}

/// GET /api/users/:id — get one user
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<User>>, (StatusCode, Json<ApiResponse<User>>)> {
    let users = state.users.lock().unwrap();

    match users.iter().find(|u| u.id == id) {
        Some(user) => Ok(Json(ApiResponse::ok(user.clone()))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(format!("User '{}' not found", id))),
        )),
    }
}

/// POST /api/users — create a new user
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<ApiResponse<User>>) {
    // Validate
    if payload.name.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<User>::error("Name cannot be empty")));
    }
    if !payload.email.contains('@') {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<User>::error("Invalid email")));
    }

    // Check for duplicate email
    {
        let users = state.users.lock().unwrap();
        if users.iter().any(|u| u.email == payload.email) {
            return (StatusCode::CONFLICT, Json(ApiResponse::<User>::error("Email already exists")));
        }
    }

    let user = User {
        id:         Uuid::new_v4().to_string(),
        name:       payload.name.trim().to_string(),
        email:      payload.email.trim().to_lowercase(),
        role:       payload.role.unwrap_or(UserRole::Member),
        created_at: Utc::now(),
    };

    state.users.lock().unwrap().push(user.clone());
    tracing::info!("Created user: {} ({})", user.name, user.id);

    (StatusCode::CREATED, Json(ApiResponse::ok(user)))
}
