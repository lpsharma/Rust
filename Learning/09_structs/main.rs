// Lesson 09: Structs

// #[derive(Debug)] lets us print with {:?}
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// Methods live in impl blocks
impl Rectangle {
    // Associated function (no self) — acts like a constructor
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height } // field shorthand
    }

    fn square(size: f64) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // Method — takes &self (immutable borrow)
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // Takes another Rectangle by reference
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

impl User {
    fn new(username: String, email: String, age: u32) -> User {
        User { username, email, age, active: true }
    }

    fn greet(&self) {
        println!("Hello, I'm {} ({})", self.username, self.email);
    }

    fn deactivate(&mut self) {
        self.active = false;
        println!("{} has been deactivated.", self.username);
    }

    fn summary(&self) -> String {
        format!("{}  |  age: {}  |  active: {}", self.username, self.age, self.active)
    }
}

// Tuple struct — like a struct but fields are unnamed
#[derive(Debug)]
struct Color(u8, u8, u8);

// Unit-like struct — no fields (useful for traits)
struct AlwaysEqual;

fn main() {
    // --- Rectangle ---
    let rect1 = Rectangle::new(10.0, 5.0);
    println!("rect1: {:?}", rect1);
    println!("Area: {}", rect1.area());
    println!("Perimeter: {}", rect1.perimeter());
    println!("Is square: {}", rect1.is_square());

    let sq = Rectangle::square(4.0);
    println!("\nsq: {:?}", sq);
    println!("Is square: {}", sq.is_square());

    let rect2 = Rectangle::new(8.0, 4.0);
    println!("\nCan rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    // --- User ---
    let mut user1 = User::new(
        String::from("alice"),
        String::from("alice@example.com"),
        30,
    );
    user1.greet();
    println!("{}", user1.summary());
    user1.deactivate();
    println!("{}", user1.summary());

    // Struct update syntax
    let user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        ..User::new(String::from("_"), String::from("_"), 25)
    };
    println!("\nuser2: {}", user2.summary());

    // --- Tuple struct ---
    let red = Color(255, 0, 0);
    println!("\nRed: ({}, {}, {})", red.0, red.1, red.2);
    println!("Red: {:?}", red);

    // --- Unit-like struct ---
    let _unit = AlwaysEqual;
}
