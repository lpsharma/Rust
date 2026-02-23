# Lesson 04 — Functions

## What You'll Learn
- Defining and calling functions
- Parameters and type annotations
- Return values (with and without `return`)
- Statements vs expressions — a key Rust distinction

---

## Key Concepts

### Function Definition
```rust
fn function_name(param: Type) -> ReturnType {
    // body
}
```
- Parameters **must** have type annotations
- Return type follows `->`
- If no return value, return type is `()` (unit) and can be omitted

### Statements vs Expressions
This is crucial in Rust:
- A **statement** performs an action but does NOT return a value: `let x = 5;`
- An **expression** evaluates to a value: `5 + 6`, `{ let x = 3; x + 1 }`

The **last expression** in a block is automatically returned — **no semicolon!**

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b   // ← no semicolon! This is the return value
}
```

Adding a semicolon makes it a statement, which returns `()` — causing a type error.

### Explicit `return`
Use `return` for early returns:
```rust
fn is_positive(n: i32) -> bool {
    if n < 0 {
        return false; // early return
    }
    true // implicit return
}
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Write a function `square(n: i32) -> i32` that returns n².
2. Write a function `is_even(n: i32) -> bool`.
3. Write a function that takes a temperature in Celsius and returns Fahrenheit.

---

## 📌 Key Takeaways
- All parameters need type annotations
- Last expression (no `;`) is the implicit return value
- Use `return` for early exits
- Rust makes a sharp distinction between *statements* and *expressions*
