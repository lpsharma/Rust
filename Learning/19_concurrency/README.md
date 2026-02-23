# Lesson 19 — Concurrency

## What You'll Learn
- Spawning threads with `std::thread`
- Passing data between threads with channels (`mpsc`)
- Shared state with `Mutex<T>` and `Arc<T>`
- Why Rust's ownership prevents data races at compile time

---

## Spawning Threads

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from a thread!");
});

handle.join().unwrap(); // wait for thread to finish
```

### Moving Data into Threads

```rust
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", v); // v is moved into the closure
});
handle.join().unwrap();
```

`move` is required — threads can outlive the scope they're created in.

---

## Channels — Message Passing

```rust
use std::sync::mpsc; // multi-producer, single-consumer

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(String::from("hello")).unwrap();
});

let received = rx.recv().unwrap();
println!("{}", received);
```

- `send()` — send a value (non-blocking)
- `recv()` — receive a value (blocks until message arrives)
- Clone `tx` for multiple producers: `let tx2 = tx.clone()`

---

## Shared State — `Arc<T>` + `Mutex<T>`

- `Arc<T>` — Atomic Reference Counting (thread-safe `Rc`)
- `Mutex<T>` — Mutual Exclusion (only one thread accesses at a time)

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));

let counter_clone = Arc::clone(&counter);
let handle = thread::spawn(move || {
    let mut num = counter_clone.lock().unwrap();
    *num += 1;
}); // lock is released when `num` goes out of scope

handle.join().unwrap();
println!("{}", *counter.lock().unwrap());
```

---

## Rust's Compile-Time Guarantees

- `Send` trait: safe to transfer between threads
- `Sync` trait: safe to share references between threads
- The compiler won't let you share non-`Send`/`Sync` types across threads!

This is "fearless concurrency" — race conditions are caught at compile time.

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Spawn 5 threads, each printing its thread number.
2. Use a channel to sum numbers computed in parallel.
3. Use `Arc<Mutex<Vec<i32>>>` to collect results from multiple threads.

---

## 📌 Key Takeaways
- `thread::spawn` + `move` closure for new threads
- Channels (`mpsc`) for safe message passing between threads
- `Arc<Mutex<T>>` for shared mutable state across threads
- Rust's type system prevents data races at compile time — "fearless concurrency"
