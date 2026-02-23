# Rust Learning Curriculum — Zero to Proficient

Welcome, Lalit! This curriculum takes you from absolute beginner to confident Rust programmer through 20 progressive lessons. Each folder contains a `README.md` (the lesson) and `main.rs` (working code).

---

## How to Use This Curriculum

Each lesson folder has:
- **`README.md`** — Read this first! Concepts, syntax, key takeaways
- **`main.rs`** — Working code with comments. Run it, modify it, experiment!

To run any lesson:
```bash
cd <lesson-folder>
rustc main.rs && ./main
```

Or with Cargo (recommended for lessons 12+):
```bash
cargo new my_practice && cd my_practice
# Copy the lesson's main.rs into src/main.rs
cargo run
```

---

## Curriculum Map

| # | Topic | Key Concepts |
|---|-------|-------------|
| 01 | Hello World | `main`, `println!`, macros |
| 02 | Variables & Mutability | `let`, `mut`, `const`, shadowing |
| 03 | Data Types | integers, floats, bool, char, tuples, arrays |
| 04 | Functions | parameters, return values, statements vs expressions |
| 05 | Control Flow | `if`, `loop`, `while`, `for`, ranges |
| 06 | **Ownership** ⭐ | ownership rules, move, clone, Copy trait |
| 07 | **References & Borrowing** ⭐ | `&`, `&mut`, borrow checker rules |
| 08 | Slices | `&str`, `&[T]`, string slices |
| 09 | Structs | defining, methods, `impl`, associated functions |
| 10 | Enums | variants with data, `Option<T>`, `match` |
| 11 | Pattern Matching | destructuring, guards, `@` bindings, `if let` |
| 12 | Modules & Crates | `mod`, `pub`, `use`, Cargo dependencies |
| 13 | Collections | `Vec`, `HashMap`, `HashSet` |
| 14 | Error Handling | `Result<T,E>`, `?` operator, custom errors |
| 15 | Generics & Traits | generic functions/structs, trait bounds |
| 16 | Lifetimes | lifetime annotations, elision, `'static` |
| 17 | Closures & Iterators | `Fn`/`FnMut`/`FnOnce`, `map`/`filter`/`fold` |
| 18 | Smart Pointers | `Box`, `Rc`, `RefCell`, `Rc<RefCell<T>>` |
| 19 | Concurrency | threads, channels, `Arc<Mutex<T>>` |
| 20 | Async/Await | `Future`, `async`/`await`, tokio |

---

## Learning Path

```
Beginner  → Lessons 01–05  (syntax basics)
Core      → Lessons 06–08  (ownership — the heart of Rust ❤️)
OOP-like  → Lessons 09–11  (structs, enums, patterns)
Real code → Lessons 12–14  (modules, collections, errors)
Advanced  → Lessons 15–17  (generics, traits, lifetimes, iterators)
Systems   → Lessons 18–20  (smart pointers, concurrency, async)
```

---

## Tips for Success

1. **Don't skip Lessons 06–07.** Ownership is what makes Rust unique. Everything else builds on it.
2. **Read the error messages.** Rust's compiler errors are famously helpful — treat them as learning tools.
3. **Experiment!** After reading each lesson, modify the `main.rs` and see what breaks (and why).
4. **Try the exercises** at the bottom of each `README.md`.
5. **Install Rust**: https://rustup.rs — then you'll have `rustc` and `cargo`.

---

## Resources
- [The Rust Book](https://doc.rust-lang.org/book/) — official, free, excellent
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) — interactive exercises
- [crates.io](https://crates.io) — Rust package registry

Happy learning! 🦀
