# Lesson 02 — Variables & Mutability

## What You'll Learn
- Declaring variables with `let`
- Why variables are **immutable by default**
- Making variables mutable with `mut`
- Constants with `const`
- **Shadowing** — reusing a variable name

---

## Key Concepts

### Immutable by Default
```rust
let x = 5;
x = 6; // ❌ ERROR: cannot assign twice to immutable variable
```

Rust makes you opt-in to mutability. This prevents accidental changes.

### Mutable Variables
```rust
let mut x = 5;
x = 6; // ✅ OK
```

### Constants
```rust
const MAX_POINTS: u32 = 100_000;
```
- Always immutable (no `mut` allowed)
- Type annotation is **required**
- Named in SCREAMING_SNAKE_CASE
- Can be declared in any scope, including global

### Shadowing
```rust
let x = 5;
let x = x + 1;     // x is now 6 — new variable, same name
let x = x * 2;     // x is now 12
```

Shadowing lets you transform a value while keeping the name. Unlike `mut`, it can also change the type!

```rust
let spaces = "   ";      // &str
let spaces = spaces.len(); // usize — different type, same name!
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Create a mutable variable `score` starting at 0, then add 10 to it.
2. Create a constant `PI` with value `3.14159` and use it in a print.
3. Shadow a variable to convert a temperature from Celsius to Fahrenheit.

---

## 📌 Key Takeaways
- Variables are **immutable by default** — this is a safety feature
- Use `mut` to make a variable mutable
- `const` is for compile-time constants (type annotation required)
- **Shadowing** lets you redeclare a variable (even with a different type)
