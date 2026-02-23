# Lesson 15 — Generics & Traits

## What You'll Learn
- Generic functions and structs
- Trait definitions and implementations
- Trait bounds (constraining generics)
- Default implementations
- Common standard traits: `Display`, `Debug`, `Clone`, `PartialOrd`

---

## Generics

Generics let you write code that works for multiple types:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
```

Generic structs:
```rust
struct Pair<T> {
    first: T,
    second: T,
}
```

---

## Traits

Traits define shared behavior (like interfaces in other languages):

```rust
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn preview(&self) -> String {
        format!("Read more: {}", self.summarize())
    }
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, &self.content[..50])
    }
}
```

---

## Trait Bounds

Constrain what types a generic can accept:

```rust
// Only works for types that implement Display
fn print_item<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

// Multiple bounds with +
fn print_debug_display<T: std::fmt::Display + std::fmt::Debug>(item: T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

// where clause (cleaner for complex bounds)
fn some_function<T, U>(t: &T, u: &U) -> String
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug,
{
    ...
}
```

---

## `impl Trait` in Function Signatures

```rust
// Parameter: "any type that implements Summary"
fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

// Return type: "some type that implements Summary"
fn make_summarizable() -> impl Summary {
    Article { ... }
}
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Define a `Drawable` trait with an `area()` and `draw()` method.
2. Implement it for `Circle` and `Square`.
3. Write a function `print_area<T: Drawable>(shape: &T)`.

---

## 📌 Key Takeaways
- Generics allow reuse across types without sacrificing type safety
- Traits define shared behavior — like interfaces
- Trait bounds constrain what types can be used
- Default implementations provide fallback behavior
- `impl Trait` is syntactic sugar for trait bounds in function signatures
