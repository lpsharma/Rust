// Lesson 12: Modules & Crates

// --- Defining modules inline ---
mod math {
    // Private by default
    fn helper_double(n: i32) -> i32 {
        n * 2
    }

    // pub makes it accessible from outside the module
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn double(n: i32) -> i32 {
        helper_double(n) // can use private fn inside module
    }

    // Nested module
    pub mod advanced {
        pub fn power(base: i32, exp: u32) -> i32 {
            base.pow(exp)
        }
    }
}

// --- Module with struct ---
mod geometry {
    pub struct Circle {
        pub radius: f64,
        // If we had private_data: String, it would be inaccessible from outside
    }

    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }

        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        pub fn circumference(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }

    pub fn area_rectangle(width: f64, height: f64) -> f64 {
        width * height
    }
}

// --- Bringing items into scope with `use` ---
use math::advanced::power;
use geometry::Circle;

// Renaming with `as`
use std::collections::HashMap as Map;

fn main() {
    // Without use — full path
    println!("add: {}", math::add(3, 4));
    println!("double: {}", math::double(5));

    // With use — shorter path
    println!("power: {}", power(2, 10));

    let c = Circle::new(5.0);
    println!("Circle area: {:.2}", c.area());
    println!("Circumference: {:.2}", c.circumference());

    println!("Rectangle area: {:.2}", geometry::area_rectangle(4.0, 6.0));

    // --- Using HashMap (from std) ---
    let mut scores: Map<String, i32> = Map::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    scores.insert(String::from("Charlie"), 92);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // --- Write into a String using fmt::Write trait ---
    use std::fmt::Write; // brings write! macro support for String
    let mut output = String::new();
    write!(output, "Hello from fmt::Write! scores has {} entries.", scores.len()).unwrap();
    println!("{}", output);

    // --- crate keyword refers to the current crate root ---
    // (in a multi-file project, crate:: is how you reference your own modules)
    println!("\nDone with modules lesson!");
}
