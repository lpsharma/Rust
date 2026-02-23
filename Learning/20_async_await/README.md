# Lesson 20 — Async / Await

## What You'll Learn
- What async programming is and why it matters
- `async fn` and `.await`
- The `Future` trait
- Using `tokio` as an async runtime
- Concurrent async tasks with `tokio::join!` and `tokio::spawn`

---

## Why Async?

Threads are great for CPU-bound work. But for I/O-bound work (networking, file I/O), threads waste resources waiting. Async lets you **wait for I/O without blocking a thread**.

```
Thread per connection:  [Thread] [waits...] [waits...] [handles response]
Async:                  [Task]   yields     task2 runs  [handles response]
```

---

## `async` and `await`

```rust
async fn fetch_data() -> String {
    // simulate async work
    "data".to_string()
}

async fn main_logic() {
    let data = fetch_data().await; // wait for the future
    println!("{}", data);
}
```

`async fn` returns a `Future` — it doesn't run until you `.await` it (or pass it to an executor).

---

## Runtimes

Rust's standard library doesn't include an async runtime. The most popular is **tokio**:

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() {
    // now you can use .await!
}
```

---

## Concurrent Tasks

```rust
// Run two futures concurrently (not sequentially!)
let (result1, result2) = tokio::join!(task1(), task2());

// Spawn independent tasks
let handle = tokio::spawn(async {
    // runs independently
});
handle.await.unwrap();
```

---

## `async` with Error Handling

```rust
async fn might_fail() -> Result<String, std::io::Error> {
    let content = tokio::fs::read_to_string("file.txt").await?;
    Ok(content)
}
```

---

## Note on Running This Example

This example requires `tokio`. To run it:

```bash
cargo new async_example
cd async_example
# Add to Cargo.toml: tokio = { version = "1", features = ["full"] }
# Copy main.rs content
cargo run
```

The `main.rs` here shows concepts with `std` only (no external deps needed). The comments show the tokio equivalents.

---

## Try It Yourself (with tokio)
1. Make two async functions that sleep for different durations, run them with `join!`.
2. Write an async function that returns `Result<String, Box<dyn Error>>`.
3. Use `tokio::spawn` to run 5 tasks and collect their results.

---

## 📌 Key Takeaways
- `async fn` returns a `Future` — lazy by default
- `.await` suspends the current task until the future completes
- You need an **async runtime** (tokio, async-std) to execute futures
- `join!` runs multiple futures concurrently
- `spawn` creates independent tasks (like lightweight threads)
- Async is best for I/O-bound work; threads for CPU-bound work
