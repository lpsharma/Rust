# Lesson 12 — Modules & Crates

## What You'll Learn
- Organizing code with `mod`
- Visibility with `pub`
- `use` for bringing items into scope
- The `crate` keyword
- Cargo and external crates (dependencies)

---

## Modules

Modules organize code into namespaces:

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    // Private by default — can't be called from outside
    fn helper() { ... }
}

fn main() {
    let result = math::add(1, 2); // use :: to access
}
```

## `pub` — Making Things Public

Everything in Rust is **private by default**. Use `pub` to expose:
- `pub fn` — public function
- `pub struct` — public struct (fields still private unless `pub`)
- `pub mod` — public module

## `use` — Import into Scope

```rust
use std::collections::HashMap;
use std::fmt::{self, Display};  // self = fmt itself

let mut map = HashMap::new(); // no need for full path!
```

### Renaming with `as`
```rust
use std::fmt::Result as FmtResult;
```

## Nested Paths
```rust
// Instead of:
use std::cmp::Ordering;
use std::io;
// Write:
use std::{cmp::Ordering, io};
```

## `*` Glob Import
```rust
use std::collections::*; // import everything (use sparingly!)
```

## Crates — Your Project and Dependencies

- A **crate** is the smallest compilation unit (either a library or binary)
- A **package** contains one or more crates + `Cargo.toml`
- `src/main.rs` → binary crate root
- `src/lib.rs` → library crate root

### Adding a Dependency (Cargo.toml)
```toml
[dependencies]
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
```

Then `cargo build` downloads and compiles it.

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Create a `geometry` module with `area_circle` and `area_rectangle` functions.
2. Make a struct inside a module with some public and some private fields.
3. Try importing `std::collections::HashMap` and use it.

---

## 📌 Key Takeaways
- Everything is private by default — use `pub` to expose
- Use `mod` to create namespaces
- `use` brings items into scope so you don't need full paths
- `::` is the path separator
- Cargo manages external dependencies in `Cargo.toml`
