# Lesson 13 — Collections

## What You'll Learn
- `Vec<T>` — growable arrays
- `HashMap<K, V>` — key-value store
- `HashSet<T>` — unique values
- Common collection methods

---

## `Vec<T>` — Vector

The most common collection. Like a dynamic array.

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);

// Shorthand with macro
let v = vec![1, 2, 3, 4, 5];

// Accessing
let third = &v[2];          // panics if out of bounds
let maybe = v.get(2);       // returns Option<&i32> — safe!
```

Common operations:
```rust
v.push(6);           // add to end
v.pop();             // remove from end -> Option<i32>
v.len();             // number of elements
v.is_empty();
v.contains(&3);
v.sort();
v.reverse();
v.iter()             // iterate over references
```

---

## `HashMap<K, V>`

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert(String::from("Alice"), 95);

// Access
let score = scores.get("Alice"); // Option<&i32>

// Entry API — insert if absent
scores.entry(String::from("Bob")).or_insert(80);

// Iterate
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

---

## `HashSet<T>`

```rust
use std::collections::HashSet;

let mut set: HashSet<i32> = HashSet::new();
set.insert(1);
set.insert(2);
set.insert(1); // duplicate — ignored!

set.contains(&1);  // true

// Set operations
let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
let b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
let intersection: HashSet<_> = a.intersection(&b).collect();
let union: HashSet<_> = a.union(&b).collect();
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Count word frequency in a sentence using `HashMap`.
2. Find the median of a sorted `Vec<i32>`.
3. Given two lists of numbers, find the common elements using `HashSet`.

---

## 📌 Key Takeaways
- `Vec<T>` — the go-to dynamic array; indexed access can panic, `.get()` is safe
- `HashMap<K, V>` — fast lookup by key; use `.entry().or_insert()` for conditional inserts
- `HashSet<T>` — unique elements; supports union, intersection, difference
- All collections are on the heap and grow dynamically
