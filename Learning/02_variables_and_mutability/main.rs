// Lesson 02: Variables & Mutability

const MAX_SCORE: u32 = 100_000; // constant: always immutable, type required

fn main() {
    // --- Immutable variable ---
    let x = 5;
    println!("x = {}", x);
    // x = 6; // ❌ Uncommenting this would cause a compile error!

    // --- Mutable variable ---
    let mut y = 10;
    println!("y before = {}", y);
    y = 20;
    println!("y after  = {}", y);

    // --- Constants ---
    println!("Max score: {}", MAX_SCORE);

    // --- Shadowing ---
    let z = 5;
    let z = z + 1; // shadow: z is now 6
    let z = z * 2; // shadow: z is now 12
    println!("Shadowed z = {}", z);

    // Shadowing can change the TYPE (unlike mut)
    let spaces = "   ";             // type: &str
    println!("spaces (str): '{}'", spaces);
    let spaces = spaces.len();      // type: usize — totally different!
    println!("spaces (len): {}", spaces);

    // --- Practical example: celsius to fahrenheit ---
    let temp_c = 100.0;
    let temp_f = temp_c * 9.0 / 5.0 + 32.0;
    println!("{}°C = {}°F", temp_c, temp_f);
}
