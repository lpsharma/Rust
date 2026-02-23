// Lesson 18: Smart Pointers

use std::rc::Rc;
use std::cell::RefCell;

// ============================================================
// Recursive type using Box (couldn't work without it!)
// ============================================================
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn push(self, val: i32) -> List {
        List::Cons(val, Box::new(self))
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = self;
        loop {
            match current {
                List::Cons(val, next) => {
                    result.push(*val);
                    current = next;
                }
                List::Nil => break,
            }
        }
        result
    }
}

// ============================================================
// Custom Drop implementation
// ============================================================
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: '{}'", self.data);
    }
}

// ============================================================
// Using Rc<T> for shared ownership
// ============================================================
#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

fn main() {
    println!("=== Box<T> ===\n");

    // Simple Box usage
    let b = Box::new(5);
    println!("b = {}", b);
    println!("*b = {}", *b); // dereference

    // Box auto-derefs — you can use methods directly
    let boxed_string = Box::new(String::from("hello"));
    println!("Length: {}", boxed_string.len()); // no need to deref!

    // Recursive type (couldn't be done without Box)
    let list = List::new().push(1).push(2).push(3);
    println!("List: {:?}", list.to_vec());

    // Box<dyn Trait> — trait objects
    let shapes: Vec<Box<dyn std::fmt::Display>> = vec![
        Box::new(42i32),
        Box::new("hello"),
        Box::new(3.14f64),
    ];
    for shape in &shapes {
        println!("Display: {}", shape);
    }

    // Drop is called at end of scope
    {
        let _ptr = CustomSmartPointer { data: String::from("inner scope") };
        println!("Pointer created.");
    } // dropped here!
    println!("Back to outer scope.\n");

    println!("=== Rc<T> — Reference Counted ===\n");

    let a = Rc::new(String::from("shared value"));
    println!("Count after creating a: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a); // cheap clone — just increments count
    println!("Count after cloning b: {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("Count after cloning c: {}", Rc::strong_count(&a));
        println!("a={}, b={}, c={}", a, b, c);
    } // c dropped here

    println!("Count after c dropped: {}", Rc::strong_count(&a));

    // Shared tree structure
    let leaf = Rc::new(Node { value: 3, children: vec![] });
    let branch = Rc::new(Node {
        value: 5,
        children: vec![Rc::clone(&leaf)],
    });
    println!("\nBranch value: {}", branch.value);
    println!("Leaf value: {}", leaf.value);
    println!("Leaf strong count: {}", Rc::strong_count(&leaf)); // 2

    println!("\n=== RefCell<T> — Interior Mutability ===\n");

    let data = RefCell::new(vec![1, 2, 3]);
    println!("Before: {:?}", data.borrow());

    // Mutably borrow and modify
    data.borrow_mut().push(4);
    data.borrow_mut().push(5);
    println!("After pushes: {:?}", data.borrow());

    // Multiple immutable borrows are fine
    let borrow1 = data.borrow();
    let borrow2 = data.borrow();
    println!("Two borrows: {:?} {:?}", borrow1, borrow2);
    drop(borrow1);
    drop(borrow2);

    // ============================================================
    // Rc<RefCell<T>> — shared mutable state (common pattern)
    // ============================================================
    println!("\n=== Rc<RefCell<T>> ===\n");

    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    let owner1 = Rc::clone(&shared);
    let owner2 = Rc::clone(&shared);

    // Both owners can modify the data!
    owner1.borrow_mut().push(10);
    owner2.borrow_mut().push(20);

    println!("shared: {:?}", shared.borrow()); // [1, 2, 3, 10, 20]
    println!("owner1: {:?}", owner1.borrow()); // same!
    println!("owner2: {:?}", owner2.borrow()); // same!

    // ============================================================
    // Summary table
    // ============================================================
    println!("\nSummary:");
    println!("  Box<T>:              single owner, heap, compile-time borrow checks");
    println!("  Rc<T>:               multiple owners, single-thread, immutable");
    println!("  RefCell<T>:          single owner, runtime borrow checks, mutable");
    println!("  Rc<RefCell<T>>:      multiple owners, single-thread, mutable");
    println!("  Arc<T>+Mutex<T>:     multiple owners, multi-thread, mutable (next lesson!)");
}
