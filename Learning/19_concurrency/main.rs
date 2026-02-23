// Lesson 19: Concurrency

use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

fn main() {
    println!("=== Thread Basics ===\n");

    // Spawn a thread
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
        for i in 1..=3 {
            println!("  thread: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=3 {
        println!("main: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    handle.join().unwrap(); // wait for thread to finish
    println!("Thread finished.\n");

    // ============================================================
    // Moving data into threads
    // ============================================================
    println!("=== Moving Data ===\n");

    let data = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("Thread owns data: {:?}", data);
        data.iter().sum::<i32>()
    });

    let sum = handle.join().unwrap();
    println!("Sum from thread: {}", sum);

    // ============================================================
    // Multiple threads
    // ============================================================
    println!("\n=== Multiple Threads ===\n");

    let mut handles = vec![];
    for i in 0..5 {
        let h = thread::spawn(move || {
            println!("Thread {} running", i);
            i * i // return value
        });
        handles.push(h);
    }

    let results: Vec<i32> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    println!("Squares: {:?}", results);

    // ============================================================
    // Channels — message passing
    // ============================================================
    println!("\n=== Channels (mpsc) ===\n");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["hello", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(5));
        }
    });

    // rx is an iterator — receives until all senders are dropped
    for received in rx {
        println!("Received: {}", received);
    }

    // Multiple producers
    println!("\n--- Multiple Producers ---");
    let (tx, rx) = mpsc::channel::<String>();

    for producer_id in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let msg = format!("Message from producer {}", producer_id);
            tx_clone.send(msg).unwrap();
        });
    }
    drop(tx); // drop original so rx knows when all senders are gone

    let mut messages: Vec<String> = rx.iter().collect();
    messages.sort(); // sort for deterministic output
    for msg in &messages {
        println!("  {}", msg);
    }

    // ============================================================
    // Shared State — Arc<Mutex<T>>
    // ============================================================
    println!("\n=== Arc<Mutex<T>> ===\n");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }); // MutexGuard dropped here — lock released!
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap()); // should be 10

    // ============================================================
    // Shared Vec with Arc<Mutex<Vec<T>>>
    // ============================================================
    println!("\n=== Collecting Results ===\n");

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let results_clone = Arc::clone(&results);
        let h = thread::spawn(move || {
            let value = i * i;
            results_clone.lock().unwrap().push(value);
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }

    let mut final_results = results.lock().unwrap().clone();
    final_results.sort();
    println!("Collected squares: {:?}", final_results);

    // ============================================================
    // Channel for work distribution
    // ============================================================
    println!("\n=== Work Distribution ===\n");

    let (work_tx, work_rx) = mpsc::channel::<i32>();
    let (result_tx, result_rx) = mpsc::channel::<i32>();
    let work_rx = Arc::new(Mutex::new(work_rx));

    // Spawn worker threads
    for worker_id in 0..3 {
        let rx = Arc::clone(&work_rx);
        let tx = result_tx.clone();
        thread::spawn(move || {
            loop {
                let job = {
                    let rx = rx.lock().unwrap();
                    rx.recv()
                };
                match job {
                    Ok(n) => {
                        let result = n * n;
                        println!("  Worker {} computed {}² = {}", worker_id, n, result);
                        tx.send(result).unwrap();
                    }
                    Err(_) => break, // channel closed
                }
            }
        });
    }
    drop(result_tx); // so result_rx knows when all workers finish

    // Send work
    for i in 1..=9 {
        work_tx.send(i).unwrap();
    }
    drop(work_tx); // signal workers to stop

    let mut results: Vec<i32> = result_rx.iter().collect();
    results.sort();
    println!("All results: {:?}", results);

    println!("\n✅ Concurrency lesson complete!");
}
