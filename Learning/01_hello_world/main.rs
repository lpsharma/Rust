// Lesson 01: Hello, World!
// Every Rust program starts with the main() function.

fn main() {
    // println! is a macro (notice the !)
    println!("Hello, World!");

    // {} is a format placeholder — Rust fills it in
    let name = "Lalit";
    println!("Hello, {}!", name);

    // You can have multiple placeholders
    println!("{} + {} = {}", 10, 20, 10 + 20);

    // Print without a newline using print!
    print!("No newline here... ");
    println!("but now there is!");

    // Debug format with {:?} — useful for printing complex values
    let numbers = [1, 2, 3];
    println!("Numbers: {:?}", numbers);
}
