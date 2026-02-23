// Lesson 03: Data Types

fn main() {
    // --- Integers ---
    let a: i32 = -42;
    let b: u32 = 100;
    let big: i64 = 9_000_000_000; // underscores for readability
    println!("i32: {}, u32: {}, i64: {}", a, b, big);

    // Different integer literals
    let hex = 0xFF;       // 255
    let binary = 0b1010;  // 10
    let byte: u8 = b'A';  // 65
    println!("hex: {}, binary: {}, byte: {}", hex, binary, byte);

    // --- Floating point ---
    let pi: f64 = 3.14159265358979;
    let e: f32 = 2.718;
    println!("pi = {:.2}, e = {:.3}", pi, e); // format decimal places

    // --- Boolean ---
    let is_rust_fun: bool = true;
    let is_hard: bool = false;
    println!("Rust is fun: {}, Hard: {}", is_rust_fun, is_hard);

    // --- Character (full Unicode!) ---
    let letter: char = 'R';
    let emoji: char = '🦀'; // Rust's mascot is a crab!
    println!("Letter: {}, Emoji: {}", letter, emoji);

    // --- Tuples ---
    let person: (i32, f64, char) = (30, 175.5, 'M');
    println!("Age: {}, Height: {}, Gender: {}", person.0, person.1, person.2);

    // Destructuring a tuple
    let (age, height, _gender) = person;
    println!("Destructured — age: {}, height: {}", age, height);

    // Unit tuple (empty tuple) — represents "nothing"
    let _unit: () = ();

    // --- Arrays ---
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    println!("First day: {}, Last: {}", days[0], days[6]);
    println!("Array length: {}", days.len());

    // Array with repeated values
    let zeros = [0i32; 5];
    println!("Zeros: {:?}", zeros);

    // Iterating over an array
    let scores = [85, 92, 78, 96, 88];
    let mut sum = 0;
    for score in scores {
        sum += score;
    }
    println!("Average score: {:.1}", sum as f64 / scores.len() as f64);
}
