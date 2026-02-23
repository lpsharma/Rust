// Lesson 14: Error Handling

use std::fmt;
use std::num::ParseIntError;

// ============================================================
// Custom Error Type
// ============================================================
#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    NegativeNumber(i32),
    DivisionByZero,
    OutOfRange { value: i32, min: i32, max: i32 },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ParseError(e) =>
                write!(f, "Parse error: {}", e),
            AppError::NegativeNumber(n) =>
                write!(f, "Expected positive number, got {}", n),
            AppError::DivisionByZero =>
                write!(f, "Cannot divide by zero"),
            AppError::OutOfRange { value, min, max } =>
                write!(f, "Value {} is out of range [{}, {}]", value, min, max),
        }
    }
}

// Implement From so ? operator can convert ParseIntError -> AppError
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

// ============================================================
// Functions that return Result
// ============================================================

fn parse_positive(s: &str) -> Result<u32, AppError> {
    let n: i32 = s.parse()?; // ? converts ParseIntError -> AppError via From
    if n < 0 {
        return Err(AppError::NegativeNumber(n));
    }
    Ok(n as u32)
}

fn safe_divide(a: i32, b: i32) -> Result<i32, AppError> {
    if b == 0 {
        return Err(AppError::DivisionByZero);
    }
    Ok(a / b)
}

fn clamp_value(value: i32, min: i32, max: i32) -> Result<i32, AppError> {
    if value < min || value > max {
        return Err(AppError::OutOfRange { value, min, max });
    }
    Ok(value)
}

// Chaining with ? — the real power of error handling
fn compute(input: &str, divisor: i32) -> Result<i32, AppError> {
    let n = parse_positive(input)?;   // propagate on error
    let divided = safe_divide(n as i32, divisor)?;
    let clamped = clamp_value(divided, 0, 100)?;
    Ok(clamped)
}

fn main() {
    // --- Basic Result matching ---
    let results = vec!["42", "-5", "abc", "200"];
    for s in &results {
        match parse_positive(s) {
            Ok(n) => println!("'{}' -> Ok({})", s, n),
            Err(e) => println!("'{}' -> Err({})", s, e),
        }
    }

    println!();

    // --- ? operator chaining ---
    let test_cases = [
        ("10", 2),
        ("10", 0),
        ("-5", 1),
        ("abc", 1),
        ("1000", 5),
    ];
    for (input, div) in test_cases {
        match compute(input, div) {
            Ok(val) => println!("compute({:?}, {}) = {}", input, div, val),
            Err(e)  => println!("compute({:?}, {}) = ERROR: {}", input, div, e),
        }
    }

    println!();

    // --- Convenience methods ---
    let v1 = parse_positive("42").unwrap();  // panics on Err
    println!("unwrap: {}", v1);

    let v2 = parse_positive("bad").unwrap_or(0); // default on Err
    println!("unwrap_or: {}", v2);

    let v3 = parse_positive("bad")
        .unwrap_or_else(|_| 99); // compute default from error
    println!("unwrap_or_else: {}", v3);

    // map — transform Ok value
    let doubled = parse_positive("21").map(|n| n * 2);
    println!("map doubled: {:?}", doubled);

    // and_then — chain Results
    let result = parse_positive("10")
        .and_then(|n| safe_divide(n as i32, 3).map(|v| v as u32));
    println!("and_then: {:?}", result);

    // --- Collecting Results ---
    let strings = vec!["1", "2", "3", "4", "5"];
    let numbers: Result<Vec<u32>, _> = strings.iter()
        .map(|s| parse_positive(s))
        .collect();
    println!("\nCollect all ok: {:?}", numbers);

    let mixed = vec!["1", "bad", "3"];
    let result2: Result<Vec<u32>, _> = mixed.iter()
        .map(|s| parse_positive(s))
        .collect();
    println!("Collect with error: {:?}", result2.map_err(|e| e.to_string()));

    // --- panic! for truly unrecoverable situations ---
    let v = vec![1, 2, 3];
    // v[99]; // would panic with: index out of bounds
    println!("\nUsing assert:");
    assert!(v.len() == 3, "Expected 3 elements");
    assert_eq!(v[0], 1, "First element should be 1");
    println!("Assertions passed!");
}
