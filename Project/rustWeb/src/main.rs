// ============================================================
// rustWeb — Full-Stack Rust Web App
// Stack: axum (router) + tera (templates) + serde (JSON)
// ============================================================

mod models;
mod handlers;
mod state;

use axum::{
    Router,
    routing::get,
};
use std::sync::{Arc, Mutex};
use tera::Tera;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use state::AppState;
use handlers::{pages, api};

#[tokio::main]
async fn main() {
    // --- Logging setup ---
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rustweb=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // --- Load Tera templates ---
    let tera = Tera::new("templates/**/*").expect("Failed to load templates");

    // --- In-memory "database" (Vec protected by Mutex + Arc) ---
    let app_state = Arc::new(AppState {
        tera,
        todos: Mutex::new(Vec::new()),
        users: Mutex::new(Vec::new()),
    });

    // --- Router ---
    let app = Router::new()
        // HTML page routes
        .route("/",                get(pages::index))
        .route("/todos",           get(pages::todos_page))
        .route("/users",           get(pages::users_page))
        .route("/about",           get(pages::about))
        // REST API routes — Todos
        .route("/api/todos",       get(api::list_todos).post(api::create_todo))
        .route("/api/todos/:id",   get(api::get_todo)
                                       .put(api::update_todo)
                                       .delete(api::delete_todo))
        // REST API routes — Users
        .route("/api/users",       get(api::list_users).post(api::create_user))
        .route("/api/users/:id",   get(api::get_user))
        // Static files (CSS, JS, images)
        .nest_service("/static",   ServeDir::new("static"))
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // --- Start server ---
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind port 3000");

    tracing::info!("🚀 Server running at http://127.0.0.1:3000");
    tracing::info!("   Pages:  / | /todos | /users | /about");
    tracing::info!("   API:    /api/todos | /api/users");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
