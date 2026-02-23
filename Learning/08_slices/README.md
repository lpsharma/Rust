# Lesson 08 — Slices

## What You'll Learn
- String slices (`&str`)
- Array and Vec slices (`&[T]`)
- Why `&str` is preferred over `&String` in function parameters
- Range syntax for slices

---

## String Slices

A slice is a **reference to a contiguous portion** of a collection.

```rust
let s = String::from("hello world");
let hello = &s[0..5];  // "hello"
let world = &s[6..11]; // "world"
```

Range shortcuts:
- `&s[..5]` = from start to index 5
- `&s[6..]` = from index 6 to end
- `&s[..]`  = entire string as a slice

### String Literals are Slices
```rust
let s: &str = "hello"; // &str pointing into binary
```

### Prefer `&str` in Function Signatures
```rust
// Good — works with both &String and &str
fn first_word(s: &str) -> &str { ... }

// Less flexible — only accepts &String
fn first_word(s: &String) -> &str { ... }
```

---

## Array Slices

```rust
let arr = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4]; // [2, 3, 4]
```

Use `&[T]` as parameter type to accept slices from arrays or Vecs:
```rust
fn sum(nums: &[i32]) -> i32 { ... }
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Write a function `last_word(s: &str) -> &str` that returns the last word.
2. Write `contains_zero(nums: &[i32]) -> bool`.
3. Print every other element from an array using a slice.

---

## 📌 Key Takeaways
- Slices are references to parts of a collection — they don't own data
- `&str` is a string slice; `&[T]` is an array/vec slice
- Prefer `&str` over `&String` in function parameters — more flexible
- Slices prevent invalid access; bounds are checked at runtime
