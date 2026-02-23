# rustWeb — Full-Stack Rust Web Application

A complete, working web app built with Rust. Demonstrates server-rendered HTML pages (Tera templates) alongside a JSON REST API — both served from the same axum server.

---

## Tech Stack

| Crate | Role |
|-------|------|
| **axum** | Web framework — routing, extractors, middleware |
| **tokio** | Async runtime (required by axum) |
| **tera** | Server-side HTML templates (Jinja2-like syntax) |
| **serde** | Serialize/deserialize — Rust structs ↔ JSON |
| **tower-http** | Middleware: CORS, static files, tracing |
| **uuid** | Generate unique IDs for todos and users |
| **chrono** | Timestamps with serde support |

---

## Project Structure

```
rustWeb/
├── Cargo.toml                 # Dependencies
├── src/
│   ├── main.rs                # Server setup, router, state init
│   ├── models.rs              # Todo, User, ApiResponse structs
│   ├── state.rs               # Shared AppState (Tera + in-memory DB)
│   └── handlers/
│       ├── mod.rs             # Module declarations
│       ├── pages.rs           # HTML page handlers (render Tera templates)
│       └── api.rs             # JSON API handlers (CRUD operations)
├── templates/
│   ├── base.html              # Base layout (nav, footer, CSS/JS links)
│   ├── index.html             # Home page
│   ├── todos.html             # Todos page + live API tester
│   ├── users.html             # Users management
│   └── about.html            # Tech stack + route reference
└── static/
    ├── css/style.css          # All styles
    └── js/app.js             # Shared JS utilities
```

---

## Getting Started

### 1. Install Rust (if not already)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Run the app
```bash
cd rustWeb
cargo run
```

First run downloads all dependencies (~30 sec). After that:
```
🚀 Server running at http://127.0.0.1:3000
   Pages:  / | /todos | /users | /about
   API:    /api/todos | /api/users
```

### 3. Open your browser
Navigate to **http://localhost:3000**

---

## API Reference

All API endpoints return JSON with this envelope:
```json
{
  "success": true,
  "data": { ... },
  "message": null,
  "count": 3
}
```

### Todos

```bash
# List all todos
curl http://localhost:3000/api/todos

# Create a todo
curl -X POST http://localhost:3000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "description": "Complete the 20-lesson curriculum"}'

# Get one todo
curl http://localhost:3000/api/todos/<id>

# Update a todo (mark complete)
curl -X PUT http://localhost:3000/api/todos/<id> \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'

# Delete a todo
curl -X DELETE http://localhost:3000/api/todos/<id>
```

### Users

```bash
# List all users
curl http://localhost:3000/api/users

# Create a user
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Lalit", "email": "lalit@example.com", "role": "admin"}'

# Get one user
curl http://localhost:3000/api/users/<id>
```

---

## Key Rust Patterns to Study

### 1. Shared State with `Arc<Mutex<T>>`
```rust
// main.rs — state is created once and cloned cheaply into every handler
let app_state = Arc::new(AppState {
    tera,
    todos: Mutex::new(Vec::new()),
    users: Mutex::new(Vec::new()),
});
```

### 2. axum Extractors — typed data from requests
```rust
// The compiler verifies these types match what the route expects
pub async fn create_todo(
    State(state): State<Arc<AppState>>,  // shared app state
    Json(payload): Json<CreateTodo>,      // deserialize JSON body
) -> (StatusCode, Json<ApiResponse<Todo>>) { ... }
```

### 3. Tera Templates — Rust data → HTML
```rust
let mut ctx = Context::new();
ctx.insert("todos", &todos);   // Vec<Todo> is serialized automatically
state.tera.render("todos.html", &ctx)
```

### 4. Error Handling with `Result`
```rust
// Handlers return Result — axum maps Err to HTTP error responses
pub async fn get_todo(...) -> Result<Json<...>, (StatusCode, Json<...>)> {
    match todos.iter().find(|t| t.id == id) {
        Some(todo) => Ok(Json(ApiResponse::ok(todo.clone()))),
        None       => Err((StatusCode::NOT_FOUND, Json(ApiResponse::error("Not found")))),
    }
}
```

### 5. `#[derive(Serialize, Deserialize)]` — zero-boilerplate JSON
```rust
#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}
// That's it — axum's Json extractor handles marshaling automatically!
```

---

## What to Extend Next

- **Persistence** — swap `Mutex<Vec<T>>` for SQLite using `sqlx`
- **Authentication** — add JWT tokens with `jsonwebtoken` crate
- **Forms** — handle HTML form submissions (`application/x-www-form-urlencoded`)
- **Validation** — use the `validator` crate for input validation
- **Testing** — write integration tests with `axum::test`
- **Deployment** — build with `cargo build --release` → single binary, ~5MB
