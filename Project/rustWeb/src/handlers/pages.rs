// ============================================================
// handlers/pages.rs — HTML page handlers
// These render Tera templates and return HTML responses
// ============================================================

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    http::StatusCode,
};
use std::sync::Arc;
use tera::Context;

use crate::state::AppState;

type AppResult = Result<Html<String>, (StatusCode, String)>;

/// Helper: render a Tera template with a given context
fn render(state: &AppState, template: &str, ctx: Context) -> AppResult {
    state.tera
        .render(template, &ctx)
        .map(Html)
        .map_err(|e| {
            tracing::error!("Template error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })
}

// ------------------------------------------------------------
// GET /  — Home page
// ------------------------------------------------------------
pub async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos  = state.todos.lock().unwrap();
    let users  = state.users.lock().unwrap();

    let mut ctx = Context::new();
    ctx.insert("title",      "rustWeb — Home");
    ctx.insert("page",       "home");
    ctx.insert("todo_count", &todos.len());
    ctx.insert("user_count", &users.len());

    render(&state, "index.html", ctx)
}

// ------------------------------------------------------------
// GET /todos  — Todos page (server-rendered list)
// ------------------------------------------------------------
pub async fn todos_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos = state.todos.lock().unwrap().clone();

    let completed   = todos.iter().filter(|t| t.completed).count();
    let pending     = todos.len() - completed;

    let mut ctx = Context::new();
    ctx.insert("title",     "rustWeb — Todos");
    ctx.insert("page",      "todos");
    ctx.insert("todos",     &todos);
    ctx.insert("completed", &completed);
    ctx.insert("pending",   &pending);

    render(&state, "todos.html", ctx)
}

// ------------------------------------------------------------
// GET /users  — Users page
// ------------------------------------------------------------
pub async fn users_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let users = state.users.lock().unwrap().clone();

    let mut ctx = Context::new();
    ctx.insert("title", "rustWeb — Users");
    ctx.insert("page",  "users");
    ctx.insert("users", &users);

    render(&state, "users.html", ctx)
}

// ------------------------------------------------------------
// GET /about
// ------------------------------------------------------------
pub async fn about(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", "rustWeb — About");
    ctx.insert("page",  "about");

    render(&state, "about.html", ctx)
}
