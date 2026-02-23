// Lesson 16: Lifetimes

// ============================================================
// Functions with lifetime annotations
// ============================================================

// The returned reference is valid as long as BOTH x and y are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Elision: compiler can figure this out — no annotation needed!
// The output borrows from the only input, so it's unambiguous.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

// When returning from self, lifetime of output = lifetime of self
fn make_greeting(name: &str, prefix: &str) -> String {
    // Returns owned String — no lifetime needed!
    format!("{}, {}!", prefix, name)
}

// ============================================================
// Struct with lifetime annotation
// ============================================================

// ImportantExcerpt can't outlive the `part` reference
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Lifetime elision applies here — output tied to &self
    fn level(&self) -> &str {
        "high"
    }

    fn announce(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part // returns a slice of the stored reference
    }
}

// ============================================================
// Generic + Trait + Lifetime together
// ============================================================

use std::fmt::Display;

fn longest_with_context<'a, T>(
    x: &'a str,
    y: &'a str,
    context: T,
) -> &'a str
where
    T: Display,
{
    println!("Context: {}", context);
    if x.len() > y.len() { x } else { y }
}

// ============================================================
// 'static lifetime
// ============================================================

fn get_static_str() -> &'static str {
    "I am in the binary — valid forever!"
}

fn main() {
    // --- longest function ---
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("Longest: '{}'", result);
    }
    // Note: `result` can only be used while BOTH string1 and string2 are valid

    // --- first_word (elided lifetime) ---
    let sentence = String::from("hello beautiful world");
    let word = first_word(&sentence);
    println!("First word: '{}'", word);
    // sentence still valid (word borrows from it)

    // --- Struct with lifetime ---
    let novel = String::from(
        "Call me Ishmael. Some years ago..."
    );
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = &novel[..i];
    }
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Excerpt: {:?}", excerpt);
    println!("Level: {}", excerpt.level());
    println!("Announced: '{}'", excerpt.announce("Big news!"));

    // --- Combine generics + traits + lifetimes ---
    let s1 = String::from("long string");
    let s2 = "short";
    let result = longest_with_context(s1.as_str(), s2, 42);
    println!("Longest with context: '{}'", result);

    // --- 'static lifetime ---
    let s: &'static str = "This is a string literal";
    println!("Static: {}", s);
    println!("From function: {}", get_static_str());

    // --- Demonstrating why lifetimes matter ---
    println!("\nLifetime summary:");
    println!("  References must never outlive the data they point to.");
    println!("  Rust checks this at compile time — zero runtime cost!");
}
