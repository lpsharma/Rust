# Lesson 16 — Lifetimes

## What You'll Learn
- What lifetimes are and why they exist
- Lifetime annotation syntax
- Lifetime elision rules
- Lifetimes in structs
- The `'static` lifetime

---

## The Problem Lifetimes Solve

Rust must ensure references are always valid. Lifetimes tell the compiler how long references live:

```rust
// ❌ This won't compile — dangling reference!
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // s is dropped here, but we're returning a ref to it!
}
```

## Lifetime Annotation Syntax

```rust
&'a str          // a reference with lifetime 'a
&'a mut String   // a mutable reference with lifetime 'a
```

Annotations describe relationships between lifetimes — they don't change how long things live.

## Lifetime Annotations in Functions

```rust
// The returned reference lives as long as the SHORTER of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

The `'a` says: "the returned reference is valid for as long as both x and y are valid."

## Lifetime Elision

Many common patterns let the compiler infer lifetimes:

**Rule 1:** Each parameter gets its own lifetime.
**Rule 2:** If there's one input lifetime, it's assigned to output.
**Rule 3:** If one parameter is `&self` or `&mut self`, its lifetime is assigned to output.

```rust
// These are equivalent:
fn first_word(s: &str) -> &str { ... }
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

## Lifetimes in Structs

When a struct holds references, it needs lifetime annotations:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,  // the struct can't outlive this reference
}
```

## `'static` Lifetime

`'static` means the reference is valid for the entire program duration:

```rust
let s: &'static str = "I live forever"; // string literals are 'static
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Write a function `shorter<'a>(x: &'a str, y: &'a str) -> &'a str`.
2. Create a struct `Parser<'a>` holding a `&'a str` and a method `parse`.
3. Try returning a reference to a local variable — observe the error.

---

## 📌 Key Takeaways
- Lifetimes prevent dangling references — the borrow checker enforces them
- `'a` annotations describe relationships, not actual durations
- Most lifetimes are elided — you only need to annotate ambiguous cases
- Structs that hold references need lifetime parameters
- `'static` = lives for the entire program (string literals, global constants)
