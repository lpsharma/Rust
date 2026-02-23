// Lesson 20: Async / Await
//
// This file demonstrates async concepts using only the standard library.
// To use real async I/O, add tokio to Cargo.toml and see the comments below.
//
// To run with tokio:
//   1. cargo new async_project && cd async_project
//   2. Add to Cargo.toml:
//        [dependencies]
//        tokio = { version = "1", features = ["full"] }
//   3. Replace src/main.rs with this file's tokio version (see comments)

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// ============================================================
// Manual Future implementation (to understand the concept)
// ============================================================
struct CountdownFuture {
    count: u32,
}

impl Future for CountdownFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<String> {
        if self.count == 0 {
            Poll::Ready(String::from("Countdown complete!"))
        } else {
            self.count -= 1;
            Poll::Pending // not ready yet
        }
    }
}

// ============================================================
// Conceptual async functions (shown as sync equivalents here)
// These are the TOKIO versions you'd use in real code:
//
// #[tokio::main]
// async fn main() {
//     // --- Basic async/await ---
//     let result = fetch_user(1).await;
//     println!("{}", result);
//
//     // --- Sequential vs Concurrent ---
//     // Sequential (slow):
//     let r1 = slow_task("A").await;
//     let r2 = slow_task("B").await;
//
//     // Concurrent (fast — both run at the same time):
//     let (r1, r2) = tokio::join!(slow_task("A"), slow_task("B"));
//
//     // --- Spawn independent tasks ---
//     let handle = tokio::spawn(async {
//         slow_task("background").await
//     });
//     let result = handle.await.unwrap();
//
//     // --- Async error handling ---
//     match read_file("config.txt").await {
//         Ok(content) => println!("{}", content),
//         Err(e) => eprintln!("Error: {}", e),
//     }
// }
//
// async fn fetch_user(id: u32) -> String {
//     tokio::time::sleep(Duration::from_millis(100)).await;
//     format!("User #{}", id)
// }
//
// async fn slow_task(name: &str) -> String {
//     tokio::time::sleep(Duration::from_millis(500)).await;
//     format!("Task {} done", name)
// }
//
// async fn read_file(path: &str) -> Result<String, std::io::Error> {
//     let content = tokio::fs::read_to_string(path).await?;
//     Ok(content)
// }
// ============================================================

fn main() {
    println!("=== Understanding Async/Await ===\n");

    println!("Key concepts:\n");

    println!("1. async fn returns a Future (lazy computation)");
    println!("   async fn greet() -> String {{ ... }}");
    println!("   // Returns Future<Output=String>, doesn't execute yet!\n");

    println!("2. .await suspends and waits for completion");
    println!("   let name = greet().await;");
    println!("   // Execution pauses here until greet() is done\n");

    println!("3. async runtime executes futures");
    println!("   #[tokio::main]  // sets up the runtime");
    println!("   async fn main() {{ ... }}\n");

    println!("4. join! for concurrent execution (not sequential!)");
    println!("   let (a, b) = tokio::join!(task_a(), task_b());");
    println!("   // Both run at the same time — twice as fast!\n");

    println!("5. spawn for independent background tasks");
    println!("   let handle = tokio::spawn(async {{ expensive_work().await }});");
    println!("   let result = handle.await.unwrap();\n");

    println!("6. Async error handling works like sync");
    println!("   async fn read() -> Result<String, Error> {{");
    println!("       let s = tokio::fs::read_to_string(\"f\").await?;");
    println!("       Ok(s)");
    println!("   }}\n");

    // --- Demonstrate the Future trait manually ---
    println!("=== Manual Future (educational) ===\n");
    println!("A Future is just a type with a poll() method:");
    println!("  Poll::Pending  — not ready yet, try again later");
    println!("  Poll::Ready(v) — done! here's the value\n");

    println!("CountdownFuture starts at 3:");
    let mut future = CountdownFuture { count: 3 };
    use std::task::{RawWaker, RawWakerVTable, Waker};
    use std::ptr;

    // Minimal waker for demonstration
    unsafe fn clone(_: *const ()) -> RawWaker { make_raw_waker() }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
    fn make_raw_waker() -> RawWaker {
        static VTABLE: RawWakerVTable = RawWakerVTable::new(
            |p| unsafe { clone(p) },
            |p| unsafe { wake(p) },
            |p| unsafe { wake_ref(p) },
            |p| unsafe { drop(p) },
        );
        RawWaker::new(ptr::null(), &VTABLE)
    }

    let waker = unsafe { Waker::from_raw(make_raw_waker()) };
    let mut cx = Context::from_waker(&waker);

    for attempt in 1..=5 {
        let pinned = Pin::new(&mut future);
        match pinned.poll(&mut cx) {
            Poll::Pending  => println!("  Attempt {}: Pending...", attempt),
            Poll::Ready(v) => {
                println!("  Attempt {}: Ready! '{}'", attempt, v);
                break;
            }
        }
    }

    println!("\nTo use real async with tokio, see the comments in this file!");
    println!("Add to Cargo.toml: tokio = {{ version = \"1\", features = [\"full\"] }}");
}
