# Lesson 05 — Control Flow

## What You'll Learn
- `if` / `else if` / `else`
- `if` as an expression
- `loop`, `while`, `for`
- `break` with a value from `loop`
- Iterating with ranges and iterators

---

## Key Concepts

### `if` as an Expression
In Rust, `if` is an **expression** — it returns a value!

```rust
let number = 7;
let description = if number > 0 { "positive" } else { "non-positive" };
```

Both branches must return the same type.

### `loop` — infinite loop with `break` value
```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2; // returns 20 from the loop!
    }
};
```

### `while`
```rust
let mut n = 3;
while n != 0 {
    println!("{}", n);
    n -= 1;
}
```

### `for` — the most idiomatic loop in Rust
```rust
let a = [10, 20, 30];
for element in a {
    println!("{}", element);
}

// Range
for i in 1..=5 {  // 1, 2, 3, 4, 5 (inclusive)
    println!("{}", i);
}

for i in 1..5 {   // 1, 2, 3, 4 (exclusive end)
    println!("{}", i);
}
```

### Loop Labels (for nested loops)
```rust
'outer: for x in 0..3 {
    for y in 0..3 {
        if x == y { continue 'outer; }
    }
}
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Print numbers 1–20 but skip multiples of 3 using `continue`.
2. Use a `loop` that asks for a number and breaks when it's even.
3. FizzBuzz: for 1–30, print "Fizz" if divisible by 3, "Buzz" if by 5, "FizzBuzz" if both.

---

## 📌 Key Takeaways
- `if` is an **expression** — both branches must have the same type
- `loop` is for infinite loops; use `break value` to return a value
- `for x in collection` is preferred over index-based loops
- `1..5` is exclusive, `1..=5` is inclusive
