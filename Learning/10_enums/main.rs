// Lesson 10: Enums & Pattern Matching

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting!"),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: rgb({}, {}, {})", r, g, b),
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // US State quarter with state name
}

fn coin_value(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {}!", state);
            25
        }
    }
}

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1..=9 => "single digit",
        10..=99 => "double digit",
        _ => "large number", // _ is catch-all
    }
}

fn main() {
    // --- Basic enum ---
    let dir = Direction::North;
    println!("Direction: {:?}", dir);

    match dir {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East  => println!("Heading East!"),
        Direction::West  => println!("Heading West!"),
    }

    // --- Enum with data ---
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
    ];
    for msg in &messages {
        msg.call();
    }

    // --- Coin example ---
    let coins = vec![
        Coin::Penny,
        Coin::Nickel,
        Coin::Quarter(String::from("Alaska")),
        Coin::Dime,
    ];
    for coin in &coins {
        println!("{:?} = {} cents", coin, coin_value(coin));
    }

    // --- Option<T> ---
    let result = safe_divide(10.0, 3.0);
    match result {
        Some(val) => println!("10 / 3 = {:.4}", val),
        None => println!("Cannot divide by zero"),
    }

    let bad = safe_divide(5.0, 0.0);
    match bad {
        Some(val) => println!("Result: {}", val),
        None => println!("Division by zero!"),
    }

    // --- if let (concise for one variant) ---
    let favorite_color: Option<&str> = Some("blue");
    if let Some(color) = favorite_color {
        println!("Favorite color: {}", color);
    }

    // --- while let ---
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    // --- Match with ranges and guards ---
    for n in [-5, 0, 3, 42, 150] {
        println!("{} is a {}", n, describe_number(n));
    }

    // --- Option methods (very useful!) ---
    let some_val: Option<i32> = Some(42);
    let no_val: Option<i32> = None;

    println!("unwrap_or: {}", no_val.unwrap_or(0));
    println!("is_some: {}", some_val.is_some());
    println!("is_none: {}", no_val.is_none());
}
