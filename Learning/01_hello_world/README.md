# Lesson 01 — Hello, World!

## What You'll Learn
- How Rust programs are structured
- The `main` function — every Rust program starts here
- Macros vs functions (`println!` is a macro)
- How to compile and run with `rustc` or `cargo`

---

## Key Concepts

### The `main` Function
Every executable Rust program needs a `main` function. It's the entry point.

```rust
fn main() {
    // code goes here
}
```

### `println!` — a Macro
The `!` tells you it's a **macro**, not a regular function.
Macros are expanded at compile time and can take variable numbers of arguments.

```rust
println!("Hello, {}!", "Lalit");  // {} is a placeholder
```

---

## Run the Example

```bash
# Option 1: compile directly
rustc main.rs
./main

# Option 2: use Cargo (recommended for real projects)
cargo new hello_world
cd hello_world
cargo run
```

---

## Try It Yourself
1. Change the greeting to use your name.
2. Print two lines — one for greeting, one for today's date.
3. Try: `println!("{} + {} = {}", 2, 3, 2+3);` — what happens?

---

## 📌 Key Takeaways
- `fn main()` is required in every Rust binary
- `println!` is a macro (note the `!`)
- `{}` is the default format placeholder
- Use `cargo` for managing real projects — it handles building, testing, and dependencies
