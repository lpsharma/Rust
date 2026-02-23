// Lesson 05: Control Flow

fn main() {
    // --- if / else if / else ---
    let temperature = 22;
    if temperature > 30 {
        println!("It's hot!");
    } else if temperature > 15 {
        println!("It's pleasant.");
    } else {
        println!("It's cold.");
    }

    // if as an expression (like ternary in other languages)
    let is_hot = if temperature > 30 { "yes" } else { "no" };
    println!("Is it hot? {}", is_hot);

    // --- loop with break value ---
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 10; // returns 50
        }
    };
    println!("Loop result: {}", result);

    // --- while ---
    let mut n = 3;
    while n > 0 {
        println!("Countdown: {}", n);
        n -= 1;
    }
    println!("Liftoff!");

    // --- for over array ---
    let fruits = ["apple", "banana", "cherry"];
    for fruit in fruits {
        println!("Fruit: {}", fruit);
    }

    // --- for with enumerate (gives index too) ---
    for (i, fruit) in fruits.iter().enumerate() {
        println!("[{}] {}", i, fruit);
    }

    // --- for with range ---
    print!("1 to 5: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // --- FizzBuzz ---
    println!("\nFizzBuzz 1-15:");
    for i in 1..=15 {
        let output = if i % 15 == 0 {
            "FizzBuzz".to_string()
        } else if i % 3 == 0 {
            "Fizz".to_string()
        } else if i % 5 == 0 {
            "Buzz".to_string()
        } else {
            i.to_string()
        };
        print!("{} ", output);
    }
    println!();

    // --- continue ---
    print!("Skip multiples of 3: ");
    for i in 1..=10 {
        if i % 3 == 0 {
            continue; // skip this iteration
        }
        print!("{} ", i);
    }
    println!();

    // --- Nested loops with labels ---
    'outer: for x in 0..3 {
        for y in 0..3 {
            if x == 1 && y == 1 {
                println!("\nBreaking outer loop at ({}, {})", x, y);
                break 'outer;
            }
            print!("({},{}) ", x, y);
        }
    }
    println!();
}
