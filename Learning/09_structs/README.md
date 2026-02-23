# Lesson 09 — Structs

## What You'll Learn
- Defining and instantiating structs
- Field shorthand syntax
- Struct update syntax
- Tuple structs and unit-like structs
- Methods with `impl`
- Associated functions (like constructors)

---

## Defining a Struct

```rust
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}
```

## Creating an Instance

```rust
let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    age: 30,
    active: true,
};
println!("{}", user.username);
```

## Field Shorthand

```rust
fn build_user(email: String, username: String) -> User {
    User { email, username, age: 0, active: true } // shorthand!
}
```

## Struct Update Syntax

```rust
let user2 = User {
    email: String::from("bob@example.com"),
    ..user1 // take remaining fields from user1
};
```

## Methods

```rust
impl User {
    fn greet(&self) {
        println!("Hi, I'm {}", self.username);
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}
```

- `&self` — immutable reference to self (read-only method)
- `&mut self` — mutable reference (modifying method)
- `self` — takes ownership (rare)

## Associated Functions (Constructors)

```rust
impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    fn square(size: f64) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
let r = Rectangle::new(5.0, 3.0);
```

---

## Run the Example
```bash
rustc main.rs && ./main
```

---

## Try It Yourself
1. Add a `full_name(&self) -> String` method to a `Person` struct.
2. Add a `is_adult(&self) -> bool` method.
3. Create a `Circle` struct with `area()` and `circumference()` methods.

---

## 📌 Key Takeaways
- Structs group related data together
- Methods are defined in `impl` blocks
- `&self` for reading, `&mut self` for modifying
- Associated functions (no `self`) are called with `::` syntax — great for constructors
- Use `#[derive(Debug)]` to print structs with `{:?}`
