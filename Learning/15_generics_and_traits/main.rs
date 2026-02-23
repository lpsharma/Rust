// Lesson 15: Generics & Traits

use std::fmt;

// ============================================================
// Generic function
// ============================================================
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("Largest is first = {}", self.first);
        } else {
            println!("Largest is second = {}", self.second);
        }
    }
}

// ============================================================
// Trait definition
// ============================================================
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // Default implementation
        format!("(Read more from {}...)", self.summarize_author())
    }
}

trait Area {
    fn area(&self) -> f64;
    fn name(&self) -> &str;

    // Default using another method
    fn describe(&self) -> String {
        format!("{} with area {:.2}", self.name(), self.area())
    }
}

// ============================================================
// Structs implementing traits
// ============================================================
#[derive(Debug)]
struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} — {}", self.title, self.author,
            &self.content[..self.content.len().min(30)])
    }
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Uses default summarize() implementation
}

#[derive(Debug)]
struct Circle { radius: f64 }
#[derive(Debug)]
struct Rectangle { width: f64, height: f64 }
#[derive(Debug)]
struct Triangle { base: f64, height: f64 }

impl Area for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn name(&self) -> &str { "Circle" }
}

impl Area for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn name(&self) -> &str { "Rectangle" }
}

impl Area for Triangle {
    fn area(&self) -> f64 { 0.5 * self.base * self.height }
    fn name(&self) -> &str { "Triangle" }
}

// ============================================================
// Functions with trait bounds
// ============================================================

// &impl Trait syntax
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Generic with trait bound
fn print_summary<T: Summary>(item: &T) {
    println!("[Summary] {}", item.summarize());
}

// Multiple trait bounds
fn print_debug_summary<T: Summary + fmt::Debug>(item: &T) {
    println!("[Debug]   {:?}", item);
    println!("[Summary] {}", item.summarize());
}

// where clause for complex bounds
fn compare_and_display<T, U>(t: &T, u: &U)
where
    T: fmt::Display + PartialOrd,
    U: fmt::Display,
{
    println!("Comparing: {} and {}", t, u);
}

// ============================================================
// main
// ============================================================
fn main() {
    // --- Generic functions ---
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    // --- Generic struct ---
    let pair = Pair::new(5, 10);
    pair.cmp_display();

    let str_pair = Pair::new("hello", "world");
    str_pair.cmp_display();

    // --- Trait usage ---
    let article = Article {
        title: String::from("Rust is Amazing"),
        author: String::from("Lalit"),
        content: String::from("Rust combines performance with safety..."),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Excited about Rust 2024 edition!"),
    };

    notify(&article);
    notify(&tweet);

    println!("\nWith debug:");
    print_debug_summary(&article);

    // --- Shapes with Area trait ---
    let shapes: Vec<Box<dyn Area>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 4.0, height: 6.0 }),
        Box::new(Triangle { base: 3.0, height: 8.0 }),
    ];

    println!("\nShapes:");
    for shape in &shapes {
        println!("  {}", shape.describe());
    }

    // Find largest area
    let max_area = shapes.iter().map(|s| s.area()).fold(f64::MIN, f64::max);
    println!("Largest area: {:.2}", max_area);
}
