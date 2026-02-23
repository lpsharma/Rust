// Lesson 11: Advanced Pattern Matching

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64), // three sides
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "Circle",
            Shape::Rectangle { .. } => "Rectangle",
            Shape::Triangle(..) => "Triangle",
        }
    }
}

fn classify_age(age: u32) -> &'static str {
    match age {
        0 => "newborn",
        n @ 1..=12 => {
            println!("  (child, age {})", n);
            "child"
        }
        n @ 13..=17 => {
            println!("  (teen, age {})", n);
            "teenager"
        }
        n @ 18..=64 => {
            println!("  (adult, age {})", n);
            "adult"
        }
        _ => "senior",
    }
}

fn main() {
    // --- Struct destructuring ---
    let p = Point { x: 5, y: 10 };
    let Point { x, y } = p; // destructure!
    println!("x={}, y={}", x, y);

    // Destructure in match
    match (Point { x: 0, y: 5 }) {
        Point { x: 0, y } => println!("On y-axis at y={}", y),
        Point { x, y: 0 } => println!("On x-axis at x={}", x),
        Point { x, y }    => println!("At ({}, {})", x, y),
    }

    // --- Shape matching ---
    let shapes: Vec<Shape> = vec![
        Shape::Circle { radius: 3.0 },
        Shape::Rectangle { width: 4.0, height: 5.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for shape in &shapes {
        println!("{}: area = {:.2}", shape.name(), shape.area());
    }

    // --- Multiple patterns with | ---
    for n in [1, 2, 3, 7, 9, 10] {
        let label = match n {
            1 | 2 | 3 => "small",
            4..=6 => "medium",
            7 | 9 => "odd large",
            _ => "other",
        };
        println!("{} -> {}", n, label);
    }

    // --- Match guards ---
    let numbers = vec![-5, -1, 0, 3, 8, 12];
    for &n in &numbers {
        let desc = match n {
            n if n < 0 => "negative",
            0 => "zero",
            n if n % 2 == 0 => "positive even",
            _ => "positive odd",
        };
        println!("{}: {}", n, desc);
    }

    // --- @ bindings ---
    let ages = [0u32, 5, 15, 25, 70];
    for &age in &ages {
        let category = classify_age(age);
        println!("Age {}: {}", age, category);
    }

    // --- Tuple patterns ---
    let pairs = [(0, 0), (1, 0), (0, 1), (1, 1)];
    for pair in pairs {
        let desc = match pair {
            (0, 0) => "origin",
            (x, 0) | (0, x) if x != 0 => "on an axis",
            (x, y) if x == y => "on diagonal",
            _ => "somewhere else",
        };
        println!("{:?} -> {}", pair, desc);
    }

    // --- matches! macro ---
    let values = vec![1, 5, 10, 15, 20];
    let big_ones: Vec<i32> = values.into_iter()
        .filter(|&x| matches!(x, 10..=20))
        .collect();
    println!("Values 10-20: {:?}", big_ones);

    // --- Destructuring in let and function args ---
    let (a, b, c) = (10, 20, 30);
    println!("a={}, b={}, c={}", a, b, c);

    let (first, .., last) = (1, 2, 3, 4, 5);
    println!("first={}, last={}", first, last);
}
