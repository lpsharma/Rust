// Lesson 13: Collections

use std::collections::{HashMap, HashSet};

fn main() {
    // ============================================================
    // Vec<T>
    // ============================================================
    println!("=== Vec ===");

    // Create
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v: {:?}", v);

    // Macro shorthand
    let mut scores = vec![85, 92, 78, 96, 88, 72];
    println!("scores: {:?}", scores);

    // Accessing — two ways
    let third = &scores[2];               // panics if out of bounds
    println!("Third (index): {}", third);

    match scores.get(10) {               // safe — returns Option
        Some(val) => println!("Got: {}", val),
        None      => println!("Index 10 doesn't exist (safe!)"),
    }

    // Iterating
    let mut total = 0;
    for score in &scores {
        total += score;
    }
    println!("Total: {}, Avg: {:.1}", total, total as f64 / scores.len() as f64);

    // Mutating while iterating
    for score in &mut scores {
        *score += 5; // dereference to modify
    }
    println!("After +5: {:?}", scores);

    // Common methods
    scores.sort();
    println!("Sorted: {:?}", scores);
    scores.reverse();
    println!("Reversed: {:?}", scores);
    println!("Max: {}", scores.iter().max().unwrap());
    println!("Min: {}", scores.iter().min().unwrap());

    scores.pop(); // remove last
    println!("After pop: {:?}", scores);

    let removed = scores.remove(1); // remove at index
    println!("Removed index 1 ({}): {:?}", removed, scores);

    scores.dedup(); // remove consecutive duplicates
    scores.retain(|&x| x > 80); // keep only elements > 80
    println!("Scores > 80: {:?}", scores);

    // Vec of strings
    let mut words: Vec<String> = Vec::new();
    words.push(String::from("hello"));
    words.push(String::from("world"));
    words.push(String::from("rust"));
    let joined = words.join(", ");
    println!("Joined: {}", joined);

    // ============================================================
    // HashMap<K, V>
    // ============================================================
    println!("\n=== HashMap ===");

    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert(String::from("Alice"), 95);
    map.insert(String::from("Bob"), 87);
    map.insert(String::from("Charlie"), 92);

    // Get (returns Option)
    if let Some(&score) = map.get("Alice") {
        println!("Alice: {}", score);
    }

    // Overwrite
    map.insert(String::from("Bob"), 90);

    // Entry API — only insert if key doesn't exist
    map.entry(String::from("Dave")).or_insert(75);
    map.entry(String::from("Alice")).or_insert(0); // Alice already exists, no change

    // Iterate (order is not guaranteed)
    let mut names: Vec<&String> = map.keys().collect();
    names.sort(); // sort for consistent output
    for name in names {
        println!("{}: {}", name, map[name]);
    }

    // Word frequency counter
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    let mut pairs: Vec<(&&str, &i32)> = word_count.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1)); // sort by count descending
    println!("\nWord frequencies:");
    for (word, count) in pairs.iter().take(5) {
        println!("  '{}': {}", word, count);
    }

    // ============================================================
    // HashSet<T>
    // ============================================================
    println!("\n=== HashSet ===");

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(String::from("page1"));
    visited.insert(String::from("page2"));
    visited.insert(String::from("page1")); // duplicate — ignored
    println!("Visited pages: {}", visited.len()); // still 2

    println!("Visited page2? {}", visited.contains("page2"));

    // Set operations
    let a: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let b: HashSet<i32> = vec![3, 4, 5, 6, 7].into_iter().collect();

    let mut intersection: Vec<i32> = a.intersection(&b).cloned().collect();
    intersection.sort();
    println!("Intersection: {:?}", intersection);

    let mut union: Vec<i32> = a.union(&b).cloned().collect();
    union.sort();
    println!("Union: {:?}", union);

    let mut only_in_a: Vec<i32> = a.difference(&b).cloned().collect();
    only_in_a.sort();
    println!("Only in A: {:?}", only_in_a);
}
