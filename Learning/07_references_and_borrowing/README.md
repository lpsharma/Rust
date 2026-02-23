# Lesson 07 — References & Borrowing

## What You'll Learn
- References (`&`) — borrow without taking ownership
- Mutable references (`&mut`)
- The borrowing rules — enforced at compile time
- Dangling references (and why Rust prevents them)

---

## References

A reference lets you **borrow** a value without owning it:

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope but doesn't drop the String (it doesn't own it!)
```

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1); // pass a reference
println!("{} has length {}", s1, len); // s1 still valid!
```

The `&` creates a reference. The reference points to the value but doesn't own it.

---

## Mutable References

```rust
fn change(s: &mut String) {
    s.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);
```

---

## The Borrowing Rules

> **Rule 1:** At any given time, you can have *either*:
>   - Any number of **immutable** references, OR
>   - **Exactly one** mutable reference

> **Rule 2:** References must always be valid (no dangling references).

```rust
let mut s = String::from("hello");
let r1 = &s;       // ✅ immutable borrow
let r2 = &s;       // ✅ another immutable borrow
let r3 = &mut s;   // ❌ ERROR: can't mix mutable and immutable borrows
```

```rust
let r1 = &mut s;
let r2 = &mut s;  // ❌ ERROR: two mutable borrows at once
```

**Why these rules?**
- Multiple immutable refs are safe — no one can change the data
- One mutable ref ensures no data races

---

## Dangling References (prevented by Rust)

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s // ❌ ERROR: s is dropped here, reference would be invalid!
}
```

Rust won't compile this. Return the `String` itself instead.

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Write a function that takes a `&String` and returns the first word.
2. Try creating two mutable references to the same variable — observe the error.
3. Write a function that appends " Rust!" to a `&mut String`.

---

## 📌 Key Takeaways
- `&T` borrows without ownership — value not dropped when ref goes out of scope
- `&mut T` allows mutation but only **one mutable borrow at a time**
- These rules eliminate data races at compile time
- Rust guarantees references are never dangling
