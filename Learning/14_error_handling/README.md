# Lesson 14 — Error Handling

## What You'll Learn
- `Result<T, E>` — the primary error type
- The `?` operator for ergonomic error propagation
- `panic!` and when to use it
- Creating custom error types
- `unwrap`, `expect`, and safer alternatives

---

## `Result<T, E>`

```rust
enum Result<T, E> {
    Ok(T),    // success value
    Err(E),   // error value
}
```

```rust
use std::fs;

let content = fs::read_to_string("file.txt");
match content {
    Ok(text) => println!("{}", text),
    Err(e)   => println!("Error: {}", e),
}
```

## Handling Results

```rust
// unwrap — panics on Err (use only in examples/tests)
let content = fs::read_to_string("file.txt").unwrap();

// expect — panics with custom message
let content = fs::read_to_string("file.txt")
    .expect("Failed to read config file");

// unwrap_or — default value
let content = fs::read_to_string("file.txt")
    .unwrap_or(String::from("default"));

// map_err — transform the error type
// ok() — convert Result<T,E> to Option<T>
```

## The `?` Operator

The `?` operator propagates errors automatically — if `Err`, return it from the function; if `Ok`, unwrap the value.

```rust
fn read_file() -> Result<String, std::io::Error> {
    let content = fs::read_to_string("file.txt")?; // ? propagates error
    Ok(content)
}
```

`?` can only be used in functions that return `Result` (or `Option`).

## `panic!`

For unrecoverable errors:
```rust
panic!("Something went terribly wrong: {}", reason);
```

When to use `panic!`:
- Tests (`assert!`, `assert_eq!`)
- Prototyping
- Situations that should truly never happen

Prefer `Result` for expected failures (file not found, network error, bad input).

## Custom Error Types

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Write a function `parse_positive(s: &str) -> Result<u32, String>` that fails on negative numbers.
2. Chain two functions that each return `Result` using `?`.
3. Add an `InvalidAge` variant to a custom error enum.

---

## 📌 Key Takeaways
- `Result<T, E>` is Rust's answer to exceptions — no try/catch!
- `?` makes error propagation concise — think of it as "return the error if present"
- `unwrap`/`expect` panic on `Err` — fine for prototyping, not production
- `panic!` is for unrecoverable bugs, `Result` is for expected failures
- Custom error types make your API clear about what can go wrong
