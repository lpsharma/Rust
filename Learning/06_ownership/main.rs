// Lesson 06: Ownership

fn main() {
    // --- Scope and drop ---
    {
        let s = String::from("hello"); // s is valid from here
        println!("Inside scope: {}", s);
    } // s goes out of scope — Rust calls drop(), memory freed automatically

    // --- Move semantics (heap data) ---
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2
    // println!("{}", s1); // ❌ ERROR: s1 is no longer valid
    println!("s2 after move: {}", s2);

    // --- Clone: explicit deep copy ---
    let s3 = String::from("world");
    let s4 = s3.clone(); // full copy of heap data
    println!("s3: {}, s4: {} (both valid)", s3, s4);

    // --- Copy types (stack only) ---
    let x = 42;
    let y = x; // x is COPIED — both remain valid
    println!("x: {}, y: {} (both valid, x is copied)", x, y);

    // --- Ownership and functions ---
    let greeting = String::from("Good morning");
    takes_ownership(greeting); // greeting is moved here
    // println!("{}", greeting); // ❌ ERROR if uncommented

    let num = 100;
    makes_copy(num); // num is copied
    println!("num still valid: {}", num); // ✅

    // --- Returning ownership ---
    let s5 = gives_ownership();
    println!("Got ownership of: {}", s5);

    let s6 = String::from("hello");
    let s7 = take_and_give_back(s6); // s6 moved in, ownership returned
    // println!("{}", s6); // ❌ would error
    println!("s7 (returned): {}", s7);

    // --- Returning multiple values with tuple ---
    let s8 = String::from("calculate length");
    let (s9, len) = calculate_length(s8);
    println!("'{}' has length {}", s9, len);
}

fn takes_ownership(some_string: String) {
    println!("Got ownership of: {}", some_string);
} // some_string is dropped here

fn makes_copy(some_integer: i32) {
    println!("Got a copy: {}", some_integer);
} // some_integer is dropped, but original is unaffected

fn gives_ownership() -> String {
    let s = String::from("yours now");
    s // moved to caller
}

fn take_and_give_back(s: String) -> String {
    s // take it and return it right back
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // return both so we don't lose s
}
