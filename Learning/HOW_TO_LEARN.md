# How to Learn Rust Step by Step — Using These Examples

## First: Install Rust (one-time setup)

```bash
# Go to https://rustup.rs and run:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# After install, verify:
rustc --version
cargo --version
```

---

## Your Daily Learning Ritual (30–60 min per lesson)

For each numbered folder, follow these exact steps:

### Step 1 — Read the README first (5–10 min)
Open `README.md`. Read it fully. Don't skip to the code yet.
The README explains *why* each concept exists, not just *what* it is.

### Step 2 — Read main.rs top to bottom (5 min)
Read the code like a book. The comments guide you line by line.

### Step 3 — Run it (2 min)
```bash
cd 01_hello_world
rustc main.rs && ./main
```
See it work. Match the output to the code.

### Step 4 — Break it deliberately (10 min)
Pick one line and change it. Try to guess what will happen, then run it.

| Try this | What you'll learn |
|----------|------------------|
| Remove the `mut` from a mutable variable | Why mutability is opt-in |
| Use a variable after moving it | The ownership rules |
| Return a value with a `;` at the end | Statements vs expressions |
| Access `arr[99]` on a 5-element array | How Rust panics safely |
| Remove `&` from a function parameter | How the borrow checker works |

### Step 5 — Do the exercises (10–15 min)
Each README ends with **"Try It Yourself"** challenges.
Write them from scratch in a new file — don't copy-paste.

### Step 6 — Review what the compiler tells you (ongoing)
When you get an error, read it fully. Rust's error messages often include:
- The exact problem
- Where it is
- A suggestion to fix it

---

## Lesson-by-Lesson Walkthrough

### 🟢 Lessons 01–05: Syntax Basics
These feel similar to other languages. Move through them quickly.

**01 — Hello World**
```bash
cd 01_hello_world && rustc main.rs && ./main
```
Try: Change `"Hello, World!"` to `"Hello, {name}!"` using a variable.
Understand: What does the `!` in `println!` mean? (It's a macro.)

**02 — Variables & Mutability**
```bash
cd 02_variables_and_mutability && rustc main.rs && ./main
```
Try: Uncomment the line `// x = 6;` and see the compile error. Read the message carefully.
Understand: Why does Rust make immutability the default?

**03 — Data Types**
```bash
cd 03_data_types && rustc main.rs && ./main
```
Try: Change `let a: i32 = -42;` to `let a: u32 = -42;` — why does it fail?
Understand: The difference between signed (i32) and unsigned (u32).

**04 — Functions**
```bash
cd 04_functions && rustc main.rs && ./main
```
Try: Add a semicolon after `a + b` in the `add` function. Read the error.
Understand: Why the last expression (no `;`) is the return value.

**05 — Control Flow**
```bash
cd 05_control_flow && rustc main.rs && ./main
```
Try: Write FizzBuzz for 1–100 yourself from scratch.
Understand: How `if` as an expression is different from other languages.

---

### 🔴 Lessons 06–08: Ownership (CRITICAL — don't rush these)
This is what makes Rust unique. Spend extra time here.

**06 — Ownership**
```bash
cd 06_ownership && rustc main.rs && ./main
```
The **mental model**: every value has exactly one owner. When that owner is done, the value is cleaned up automatically — no garbage collector needed.

Try these in a scratch file:
```rust
// Experiment 1: Move
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1); // Try this — what error do you get?

// Experiment 2: Clone fixes it
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{} {}", s1, s2); // Both valid!

// Experiment 3: Copy types don't move
let x = 5;
let y = x;
println!("{} {}", x, y); // Both valid! (integers are Copy)
```

Understand: The difference between types that `Copy` (stack) and types that `Move` (heap).

**07 — References & Borrowing**
```bash
cd 07_references_and_borrowing && rustc main.rs && ./main
```
The **mental model**: you can lend a value without giving it away.

The golden rule:
- As many `&` (shared/immutable) borrows as you want, OR
- Exactly one `&mut` (exclusive/mutable) borrow — never both at the same time.

Try:
```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s; // ← why does this fail?
println!("{}", r1);
```

**08 — Slices**
```bash
cd 08_slices && rustc main.rs && ./main
```
Understand: `&str` is a window into string data. It doesn't own anything.

---

### 🟡 Lessons 09–11: Structs, Enums, Patterns

**09 — Structs**
```bash
cd 09_structs && rustc main.rs && ./main
```
Try: Add a `BMI` method to a `Person` struct that has `weight_kg` and `height_m` fields.

**10 — Enums**
```bash
cd 10_enums && rustc main.rs && ./main
```
The **key insight**: Rust enums can carry data. This is much more powerful than enums in Java or C.
`Option<T>` replaces `null` — the compiler forces you to handle the missing case.

Try:
```rust
let value: Option<i32> = Some(42);
let doubled = value.map(|n| n * 2); // transform if present
println!("{:?}", doubled); // Some(84)
```

**11 — Pattern Matching**
```bash
cd 11_pattern_matching && rustc main.rs && ./main
```
Try: Write a `classify_triangle(a: f64, b: f64, c: f64) -> &str` that returns "equilateral", "isosceles", or "scalene" using `match`.

---

### 🔵 Lessons 12–14: Real-World Tools

**12 — Modules & Crates**
```bash
cd 12_modules_and_crates && rustc main.rs && ./main
```
For real projects, switch to Cargo. Create your first Cargo project:
```bash
cargo new my_project
cd my_project
cargo run     # compiles and runs
cargo build   # just compiles
cargo check   # fastest: just checks errors, no binary
```

**13 — Collections**
```bash
cd 13_collections && rustc main.rs && ./main
```
Try: Write a word frequency counter — given a sentence, print each word and how many times it appears, sorted by frequency.

**14 — Error Handling**
```bash
cd 14_error_handling && rustc main.rs && ./main
```
The **key insight**: no exceptions in Rust. Functions that can fail return `Result<T, E>`.
The `?` operator is your best friend — it propagates errors automatically.

Try: Write a function that opens a file, reads its content, and returns the number of lines, using `?` for error propagation.

---

### 🟣 Lessons 15–17: Advanced Rust

**15 — Generics & Traits**
```bash
cd 15_generics_and_traits && rustc main.rs && ./main
```
Think of traits as interfaces — they define *what a type can do*.
Generics let you write *one* function that works for *many* types.

Try: Write a `Printable` trait with a `pretty_print(&self)` method. Implement it for `i32` and `String`.

**16 — Lifetimes**
```bash
cd 16_lifetimes && rustc main.rs && ./main
```
Lifetimes are the hardest concept. The key insight: they don't change *how long* things live — they just tell the compiler about the relationships between references.

Most of the time the compiler figures them out (lifetime elision). You only annotate when there's ambiguity.

**17 — Closures & Iterators**
```bash
cd 17_closures_and_iterators && rustc main.rs && ./main
```
This is where Rust becomes genuinely elegant. Practice replacing `for` loops with iterator chains:

```rust
// for loop style:
let mut evens = vec![];
for &n in &numbers {
    if n % 2 == 0 { evens.push(n * n); }
}

// iterator style (preferred in Rust):
let evens: Vec<i32> = numbers.iter()
    .filter(|&&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect();
```

---

### ⚫ Lessons 18–20: Systems Programming

**18 — Smart Pointers**
```bash
cd 18_smart_pointers && rustc main.rs && ./main
```
Use `Box<T>` by default when you need heap allocation.
Use `Rc<RefCell<T>>` when multiple parts of your code need to share and modify data.

**19 — Concurrency**
```bash
cd 19_concurrency && rustc main.rs && ./main
```
The **key insight**: Rust's ownership rules make data races *impossible at compile time*.
`Arc<Mutex<T>>` is the standard pattern for shared mutable state across threads.

**20 — Async/Await**
For this one, create a Cargo project with tokio:
```bash
cargo new async_demo && cd async_demo
# Edit Cargo.toml, add under [dependencies]:
# tokio = { version = "1", features = ["full"] }
cargo run
```

---

## The Learning Loop (repeat for every concept)

```
Read README  →  Run code  →  Break it  →  Fix it  →  Extend it  →  Exercises
```

Never just read. Always run the code and modify it. The compiler is your teacher.

---

## Common Beginner Mistakes (and how to avoid them)

| Mistake | Fix |
|---------|-----|
| Trying to use a value after moving it | Pass a reference `&val` instead |
| Fighting the borrow checker | Ask: "who should own this?" |
| Using `unwrap()` everywhere | Handle errors properly with `match` or `?` |
| Reaching for `clone()` too often | Think about borrowing first |
| Skipping ownership lessons | Go back to lessons 06–07 |

---

## Suggested Weekly Schedule

| Week | Lessons | Focus |
|------|---------|-------|
| 1 | 01–05 | Syntax, get comfortable running Rust |
| 2 | 06–08 | Ownership — the heart of Rust |
| 3 | 09–11 | Structs, enums, pattern matching |
| 4 | 12–14 | Real Rust: modules, collections, errors |
| 5 | 15–17 | Generics, traits, iterators |
| 6 | 18–20 | Smart pointers, concurrency, async |

After 6 weeks: **build something!** A CLI tool, a small web server, or a data processor.

---

## What to Build After Completing All 20 Lessons

1. **CLI tool** — number guessing game, calculator, todo list
2. **File processor** — word counter, CSV reader/formatter
3. **Mini web server** — use `axum` or `actix-web` crate
4. **Multithreaded downloader** — parallel HTTP requests with `reqwest`
5. **Systems tool** — a simple `grep` or `ls` clone

Good luck, Lalit! 🦀
