# Lesson 06 — Ownership ⭐ (The Most Important Rust Concept)

## What You'll Learn
- The three ownership rules
- Stack vs heap memory
- Move semantics
- The `Clone` and `Copy` traits

---

## The 3 Ownership Rules

> 1. Each value in Rust has an **owner**.
> 2. There can only be **one owner at a time**.
> 3. When the owner goes out of scope, the value is **dropped** (freed).

This is how Rust achieves memory safety **without a garbage collector**.

---

## Stack vs Heap

- **Stack**: fixed size, fast, for simple values (integers, booleans, chars)
- **Heap**: dynamic size, slower, for data that grows (String, Vec, etc.)

---

## Move Semantics

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is MOVED into s2
println!("{}", s1); // ❌ ERROR! s1 is no longer valid
```

`String` is heap-allocated. Rust moves ownership rather than doing a shallow copy (to prevent double-free).

### Clone — explicit deep copy
```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // explicit deep copy — both are valid
println!("{} and {}", s1, s2); // ✅
```

## Copy Types

Stack-only types implement `Copy` — they're trivially duplicated:
```rust
let x = 5;
let y = x; // x is COPIED (not moved) — both are valid
println!("{} and {}", x, y); // ✅
```

Copy types include: all integers, floats, booleans, `char`, tuples of Copy types.

---

## Ownership and Functions

Passing to a function **moves or copies** the value:
```rust
fn takes_ownership(s: String) { /* s is valid here */ }
// After this call, the original is gone!

fn makes_copy(n: i32) { /* n is a copy */ }
// Original is still valid
```

Functions can **return** ownership back:
```rust
fn gives_ownership() -> String {
    String::from("hello") // ownership moves to caller
}
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Try to use a `String` after moving it — observe the error.
2. Use `.clone()` to fix the error.
3. Write a function that takes a `String`, modifies it, and returns it back.

---

## 📌 Key Takeaways
- **One owner at a time** — prevents data races and double-frees
- Moving transfers ownership; using after a move is a compile error
- `.clone()` for explicit deep copies of heap data
- Stack types (`i32`, `bool`, etc.) are automatically `Copy`
- This is Rust's magic — memory safety without a GC!
