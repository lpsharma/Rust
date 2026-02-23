// Lesson 17: Closures & Iterators

// ============================================================
// Custom Iterator
// ============================================================
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// ============================================================
// Higher-order function (takes a closure)
// ============================================================
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn apply_to_each<F: Fn(i32) -> i32>(nums: &[i32], f: F) -> Vec<i32> {
    nums.iter().map(|&x| f(x)).collect()
}

fn main() {
    // ============================================================
    // Closures
    // ============================================================
    println!("=== Closures ===");

    // Basic closure
    let add_one = |x: i32| x + 1;
    let square = |x: i32| x * x;
    println!("add_one(5) = {}", add_one(5));
    println!("square(4)  = {}", square(4));

    // Closure capturing environment
    let threshold = 100;
    let is_above_threshold = |x: i32| x > threshold;
    println!("200 > threshold? {}", is_above_threshold(200));
    println!("threshold still valid: {}", threshold); // not moved

    // FnMut — captures by mutable reference
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("count: {}", increment());
    println!("count: {}", increment());
    println!("count: {}", increment());
    drop(increment); // release borrow
    println!("Final count: {}", count);

    // FnOnce — moves captured value (can only call once)
    let name = String::from("Lalit");
    let greet = move || format!("Hello, {}!", name); // name is moved
    println!("{}", greet());
    // name is no longer valid here

    // Returning closures (must use Box<dyn Fn>)
    fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| x + n)
    }
    let add5 = make_adder(5);
    let add10 = make_adder(10);
    println!("add5(3) = {}, add10(3) = {}", add5(3), add10(3));

    // apply_twice
    println!("apply_twice(double, 3) = {}", apply_twice(|x| x * 2, 3)); // 12
    println!("apply_twice(add1, 5) = {}",   apply_twice(|x| x + 1, 5)); // 7

    // apply_to_each
    let nums = vec![1, 2, 3, 4, 5];
    let doubled = apply_to_each(&nums, |x| x * 2);
    println!("doubled: {:?}", doubled);

    // ============================================================
    // Iterators
    // ============================================================
    println!("\n=== Iterators ===");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map + collect
    let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("squares: {:?}", squares);

    // filter + collect
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // filter_map — filter and transform in one step
    let strings = vec!["1", "two", "3", "four", "5"];
    let parsed: Vec<i32> = strings.iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("parsed: {:?}", parsed);

    // fold
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("sum via fold: {}", sum);

    // Shortcut methods
    let total: i32 = numbers.iter().sum();
    let product: i32 = vec![1, 2, 3, 4, 5].iter().product();
    println!("sum: {}, product: {}", total, product);

    // max, min, count
    println!("max: {:?}, min: {:?}", numbers.iter().max(), numbers.iter().min());
    println!("count: {}", numbers.iter().filter(|&&x| x > 5).count());

    // enumerate
    let fruits = vec!["apple", "banana", "cherry"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("[{}] {}", i, fruit);
    }

    // zip
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];
    let combined: Vec<_> = names.iter().zip(scores.iter()).collect();
    for (name, score) in &combined {
        println!("{}: {}", name, score);
    }

    // chain
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("chained: {:?}", chained);

    // take and skip
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_five: Vec<&i32> = numbers.iter().skip(5).collect();
    println!("take(3): {:?}", first_three);
    println!("skip(5): {:?}", skip_five);

    // flat_map
    let words = vec!["hello world", "foo bar baz"];
    let all_words: Vec<&str> = words.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("flat_map words: {:?}", all_words);

    // any and all
    println!("any > 5: {}", numbers.iter().any(|&x| x > 5));
    println!("all > 0: {}", numbers.iter().all(|&x| x > 0));

    // find and position
    println!("first > 5: {:?}", numbers.iter().find(|&&x| x > 5));
    println!("position > 5: {:?}", numbers.iter().position(|&x| x > 5));

    // ============================================================
    // Custom Iterator
    // ============================================================
    println!("\n=== Custom Iterator ===");
    let counter = Counter::new(5);
    let result: Vec<u32> = counter.collect();
    println!("Counter: {:?}", result);

    // Use all the standard iterator methods on our custom iterator!
    let sum_of_pairs: u32 = Counter::new(5)
        .zip(Counter::new(5).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("Sum of paired products divisible by 3: {}", sum_of_pairs);
}
