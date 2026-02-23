# Lesson 03 — Data Types

## What You'll Learn
- Scalar types: integers, floats, booleans, characters
- Compound types: tuples and arrays
- Type annotations
- Integer overflow behavior

---

## Scalar Types

### Integers
| Type   | Size    | Range                        |
|--------|---------|------------------------------|
| `i8`   | 8-bit   | -128 to 127                  |
| `u8`   | 8-bit   | 0 to 255                     |
| `i32`  | 32-bit  | ~-2 billion to ~2 billion    |
| `u32`  | 32-bit  | 0 to ~4 billion              |
| `i64`  | 64-bit  | very large range             |
| `usize`| pointer | used for indexing            |

Default integer type is **`i32`**.

### Floating Point
```rust
let x: f64 = 3.14;   // default, 64-bit double precision
let y: f32 = 3.14;   // 32-bit single precision
```

### Boolean
```rust
let t: bool = true;
let f: bool = false;
```

### Character
```rust
let c: char = 'z';
let emoji: char = '😀'; // Rust chars are full Unicode!
```

---

## Compound Types

### Tuples — fixed-size, mixed types
```rust
let tup: (i32, f64, char) = (42, 3.14, 'R');
let (a, b, c) = tup;   // destructuring
let first = tup.0;     // index access
```

### Arrays — fixed-size, same type
```rust
let arr = [1, 2, 3, 4, 5];
let first = arr[0];
let len = arr.len();

let zeros = [0; 5]; // [0, 0, 0, 0, 0]
```

**Arrays are stack-allocated and fixed size.** Use `Vec` (lesson 13) for dynamic collections.

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Create a tuple holding a person's name length, age, and height.
2. Create an array of 7 days of the week and print the 3rd one.
3. What happens if you access `arr[10]` on a 5-element array? Try it!

---

## 📌 Key Takeaways
- Rust is **statically typed** — every value has a type known at compile time
- Default number types: `i32` for integers, `f64` for floats
- Tuples: fixed-size, mixed types, access with `.0`, `.1`, etc.
- Arrays: fixed-size, same type, stack-allocated
- Out-of-bounds array access **panics at runtime** (not undefined behavior!)
