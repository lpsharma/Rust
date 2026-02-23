# Lesson 17 — Closures & Iterators

## What You'll Learn
- Closure syntax and how they capture variables
- `Fn`, `FnMut`, `FnOnce` traits
- Iterator adapters: `map`, `filter`, `fold`, `collect`, `zip`, `take`
- Chaining iterators
- Writing your own iterator

---

## Closures

Closures are anonymous functions that can capture their environment:

```rust
let add_one = |x| x + 1;
let square = |x: i32| -> i32 { x * x };

println!("{}", add_one(5));  // 6
println!("{}", square(4));   // 16
```

### Capturing the Environment

```rust
let threshold = 10;

// Captures `threshold` by reference (Fn)
let is_big = |x| x > threshold;

println!("{}", is_big(15)); // true
println!("{}", threshold);  // still valid!
```

### Closure Traits
- `Fn` — borrows immutably (can be called multiple times)
- `FnMut` — borrows mutably (can modify captured vars)
- `FnOnce` — consumes captures (can only be called once)

---

## Iterators

Iterators produce a sequence of values lazily:

```rust
let v = vec![1, 2, 3, 4, 5];
let mut iter = v.iter(); // doesn't do any work yet!
iter.next(); // Some(&1)
iter.next(); // Some(&2)
```

### Common Adapters (lazy — chain without cost)

```rust
v.iter()
 .filter(|&&x| x % 2 == 0)    // keep evens: [2, 4]
 .map(|&x| x * x)              // square: [4, 16]
 .collect::<Vec<_>>()          // materialize: vec![4, 16]
```

```rust
// fold: reduce to single value
let sum = v.iter().fold(0, |acc, &x| acc + x);

// sum() and product() are shortcuts
let total: i32 = v.iter().sum();

// enumerate: get index + value
for (i, val) in v.iter().enumerate() { ... }

// zip: combine two iterators
let zipped: Vec<_> = a.iter().zip(b.iter()).collect();

// chain: concatenate iterators
let combined: Vec<_> = a.iter().chain(b.iter()).collect();

// take / skip
let first_three: Vec<_> = v.iter().take(3).collect();
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Use `map` to convert a `Vec<&str>` of numbers to `Vec<i32>`.
2. Use `filter` + `collect` to keep only words longer than 4 chars.
3. Use `fold` to find the maximum in a `Vec<i32>`.
4. Implement a `Counter` struct with `Iterator` for values 1..=5.

---

## 📌 Key Takeaways
- Closures capture their environment; the compiler infers the capture mode
- Iterators are **lazy** — no work happens until you consume them (`.collect()`, `.sum()`, etc.)
- Prefer iterator chains over manual `for` loops — more expressive and often faster
- `map` transforms, `filter` selects, `fold` reduces, `collect` materializes
- You can implement `Iterator` on your own types!
