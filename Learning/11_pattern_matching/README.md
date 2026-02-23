# Lesson 11 — Advanced Pattern Matching

## What You'll Learn
- Destructuring structs, enums, tuples
- Match guards
- `@` bindings
- Nested patterns
- `matches!` macro

---

## Patterns Everywhere

Patterns appear in: `match`, `if let`, `while let`, `let`, function parameters.

### Destructuring a Struct
```rust
let p = Point { x: 5, y: 10 };
let Point { x, y } = p; // destructure directly
```

### Destructuring Enum Variants
```rust
match msg {
    Message::Move { x, y } => ...,
    Message::Write(text) => ...,
}
```

### Ignoring Values with `_` and `..`
```rust
let (a, _, c) = (1, 2, 3); // ignore middle

struct Point3D { x: i32, y: i32, z: i32 }
let Point3D { x, .. } = p; // ignore y and z
```

### Match Guards — Extra Conditions
```rust
match num {
    n if n < 0  => println!("negative"),
    n if n == 0 => println!("zero"),
    _           => println!("positive"),
}
```

### `@` Bindings — Capture and Test
```rust
match age {
    n @ 1..=12  => println!("Child aged {}", n),
    n @ 13..=17 => println!("Teen aged {}", n),
    n           => println!("Adult aged {}", n),
}
```

### Multiple Patterns with `|`
```rust
match x {
    1 | 2 => println!("one or two"),
    3..=5 => println!("three to five"),
    _     => println!("something else"),
}
```

### `matches!` Macro — Returns bool
```rust
let x = 5;
println!("{}", matches!(x, 1 | 2 | 3));  // false
println!("{}", matches!(x, 4..=10));      // true
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Destructure a `Rectangle` struct in a `let` statement.
2. Write a `match` with a guard that checks if a number is even.
3. Use `@` to print "teenager aged N" for ages 13–17.

---

## 📌 Key Takeaways
- Patterns can appear in `match`, `if let`, `let`, and function params
- Destructuring pulls values out of structs, tuples, and enums
- Guards add conditions: `n if n > 0`
- `@` bindings capture a value while matching: `n @ 1..=10`
- `matches!(val, pattern)` is handy for boolean pattern checks
