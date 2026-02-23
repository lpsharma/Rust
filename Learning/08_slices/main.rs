// Lesson 08: Slices

fn main() {
    // --- String slices (&str) ---
    let s = String::from("hello world");

    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"
    println!("{} {}", hello, world);

    // Shorthand ranges
    let from_start = &s[..5];   // same as &s[0..5]
    let to_end = &s[6..];       // same as &s[6..11]
    println!("'{}' '{}'", from_start, to_end);

    // String literals are slices
    let literal: &str = "I am a slice"; // already a &str!
    println!("{}", literal);

    // Function that works on both String and &str
    let word = first_word(&s);
    println!("First word: '{}'", word);

    // Same function works with string literals
    let sentence = "the quick brown fox";
    let first = first_word(sentence);
    println!("First word of literal: '{}'", first);

    // --- Array slices ---
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // [2, 3, 4]
    println!("Array slice: {:?}", slice);
    println!("Slice length: {}", slice.len());

    // Pass array slice to a function
    let sum = sum_slice(&arr[1..3]);
    println!("Sum of arr[1..3]: {}", sum);

    // --- Slices are references — safe and bounds-checked ---
    let numbers = vec![10, 20, 30, 40, 50];
    print_first_n(&numbers, 3);
}

// &str works for both &String and string literals
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Accepts a slice of i32
fn sum_slice(slice: &[i32]) -> i32 {
    let mut total = 0;
    for &n in slice {
        total += n;
    }
    total
}

fn print_first_n(data: &[i32], n: usize) {
    println!("First {} elements: {:?}", n, &data[..n]);
}
