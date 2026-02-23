// Lesson 07: References & Borrowing

fn main() {
    // --- Immutable reference ---
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // borrow s1
    println!("'{}' has length {}", s1, len); // s1 still valid!

    // --- Mutable reference ---
    let mut s = String::from("hello");
    append_world(&mut s); // mutably borrow
    println!("After append: {}", s);

    // --- Multiple immutable refs are fine ---
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this point

    // --- Now we can mutably borrow (previous refs are gone) ---
    let r3 = &mut s;
    r3.push_str("!!!");
    println!("r3 (mutable): {}", r3);

    // --- Reference in function doesn't drop value ---
    let my_string = String::from("Rust is awesome");
    let word = first_word(&my_string);
    println!("First word: {}", word);
    println!("Original still valid: {}", my_string);

    // --- Demonstrating ref scope ends before next borrow ---
    let mut data = String::from("abc");
    {
        let borrow1 = &data;
        let borrow2 = &data;
        println!("Two immutable: '{}', '{}'", borrow1, borrow2);
    } // borrow1 and borrow2 end here

    let borrow3 = &mut data; // ✅ now safe to mutably borrow
    borrow3.push('d');
    println!("After push: {}", borrow3);
}

// Takes an immutable reference — doesn't move ownership
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but String is NOT dropped (we don't own it)

// Takes a mutable reference — can modify the value
fn append_world(s: &mut String) {
    s.push_str(", world");
}

// Returns a slice (reference) to part of the string
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i]; // return slice up to the space
        }
    }
    &s[..] // no space found, return whole string
}
