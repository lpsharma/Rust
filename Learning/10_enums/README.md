# Lesson 10 — Enums & Pattern Matching

## What You'll Learn
- Defining enums
- Enums with associated data
- The `Option<T>` enum (Rust's answer to null)
- `match` expressions
- `if let` shorthand

---

## Defining Enums

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let dir = Direction::North;
```

## Enums with Data

Each variant can hold different types of data:

```rust
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },   // named fields (struct-like)
    Write(String),              // tuple-like
    ChangeColor(u8, u8, u8),   // multiple values
}
```

This is more expressive than C-style enums!

## `Option<T>` — No Null!

Rust has no `null`. Instead:
```rust
enum Option<T> {
    Some(T),
    None,
}
```

```rust
let some_number: Option<i32> = Some(42);
let no_number: Option<i32> = None;
```

Before using an `Option`, you must **handle the None case** — the compiler forces you!

## `match` Expression

```rust
match direction {
    Direction::North => println!("Going north!"),
    Direction::South => println!("Going south!"),
    Direction::East  => println!("Going east!"),
    Direction::West  => println!("Going west!"),
}
```

`match` is **exhaustive** — you must cover all variants (or use `_` as a catch-all).

## Matching with Data

```rust
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    Message::Write(text) => println!("Write: {}", text),
    Message::ChangeColor(r, g, b) => println!("Color: {},{},{}", r, g, b),
}
```

## `if let` Shorthand

When you only care about one variant:

```rust
if let Some(value) = some_option {
    println!("Got: {}", value);
}
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Create a `Shape` enum with `Circle(f64)`, `Rectangle(f64, f64)`, `Triangle(f64, f64, f64)`.
2. Write an `area(shape: &Shape) -> f64` function using `match`.
3. Write a function `safe_divide(a: f64, b: f64) -> Option<f64>`.

---

## 📌 Key Takeaways
- Enums express "one of these possibilities" — more powerful than in other languages
- Variants can carry different kinds of data
- `Option<T>` replaces null — forces you to handle missing values
- `match` must be exhaustive — no forgetting cases!
- `if let` is a concise alternative when you only care about one variant
