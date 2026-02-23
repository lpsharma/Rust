# Lesson 18 — Smart Pointers

## What You'll Learn
- `Box<T>` — heap allocation
- `Rc<T>` — reference counting (shared ownership)
- `RefCell<T>` — interior mutability
- `Rc<RefCell<T>>` — shared mutable data
- The `Deref` and `Drop` traits

---

## `Box<T>` — Heap Allocation

Use `Box` when:
- You have a large value you want on the heap
- You need a type whose size can't be known at compile time (recursive types)
- You need a trait object (`Box<dyn Trait>`)

```rust
let b = Box::new(5); // 5 is on the heap
println!("{}", b);   // auto-dereferences
println!("{}", *b);  // manual deref
```

### Recursive Types with Box
```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```
Without `Box`, the size would be infinite.

---

## `Rc<T>` — Reference Counting

Single-threaded shared ownership:

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a);  // increments reference count
println!("Count: {}", Rc::strong_count(&a)); // 2
```

When the last `Rc` is dropped, the value is freed. No cycle detection — use `Weak` to avoid cycles.

---

## `RefCell<T>` — Interior Mutability

Enforces borrowing rules **at runtime** instead of compile time:

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);
data.borrow_mut().push(4); // runtime borrow check
println!("{:?}", data.borrow()); // immutable borrow
```

Panics if borrowing rules are violated at runtime.

---

## `Rc<RefCell<T>>` — Shared Mutable Data

The common pattern for single-threaded shared mutable state:

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
let clone1 = Rc::clone(&shared);

clone1.borrow_mut().push(4);
println!("{:?}", shared.borrow()); // [1, 2, 3, 4]
```

---

## Comparison

| Type | Ownership | Mutability | Thread Safe |
|------|-----------|------------|-------------|
| `Box<T>` | Single | Normal rules | Yes |
| `Rc<T>` | Multiple | Immutable | No |
| `Arc<T>` | Multiple | Immutable | Yes |
| `RefCell<T>` | Single | Runtime checked | No |
| `Mutex<T>` | Multiple | Runtime locked | Yes |

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Create a recursive linked list with `Box`.
2. Use `Rc<RefCell<Vec<i32>>>` shared between two "owners".
3. Demonstrate the `Deref` trait by auto-dereferencing a `Box<String>`.

---

## 📌 Key Takeaways
- `Box<T>`: simple heap allocation + single ownership
- `Rc<T>`: multiple owners, single-threaded, immutable
- `RefCell<T>`: defer borrow checking to runtime (use carefully!)
- `Rc<RefCell<T>>`: the classic single-threaded shared mutable pattern
- For multi-threading: use `Arc<T>` and `Mutex<T>` (next lesson!)
