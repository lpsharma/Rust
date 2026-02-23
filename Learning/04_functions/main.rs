// Lesson 04: Functions

fn main() {
    // Calling functions
    greet("Lalit");

    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);

    let area = rectangle_area(5.0, 3.0);
    println!("Area = {}", area);

    println!("5 is positive: {}", is_positive(5));
    println!("-3 is positive: {}", is_positive(-3));

    let category = categorize_score(85);
    println!("Score 85 is: {}", category);

    // Block as expression
    let result = {
        let x = 10;
        let y = 20;
        x + y // no semicolon: this is the value of the block
    };
    println!("Block result: {}", result);
}

// No parameters, no return value (returns ())
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Parameters with types, explicit return type
fn add(a: i32, b: i32) -> i32 {
    a + b // implicit return — no semicolon!
}

// Floating point parameters
fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

// Early return with `return` keyword
fn is_positive(n: i32) -> bool {
    if n < 0 {
        return false; // early return
    }
    true // implicit return
}

// Returning a &str (string slice)
fn categorize_score(score: u32) -> &'static str {
    if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else {
        "F"
    }
}
