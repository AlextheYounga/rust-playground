Writing concurrent code in Rust involves using threads, message-passing, and shared state concurrency. Rust's ownership model provides strong guarantees that your concurrent programs will be free of data races and other common pitfalls.

### 1. Using Threads:

Rust has a thread library that allows you to run code concurrently by creating new threads.

Here's a simple example of creating a new thread:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Execute some code in the main thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}
```

In this example, the main thread spawns a new thread and then both proceed to run concurrently. The `join` method is called on the main thread to ensure it doesn't exit before the spawned thread is finished.

### 2. Message-Passing Concurrency (Channels):

Rust provides a message-passing concurrency model through channels, allowing you to send data between threads in a safe and controlled manner.

Here's how you might use channels:

```rust
use std::sync::mpsc; // mpsc stands for multiple producer, single consumer.
use std::thread;

fn main() {
    // Create a new channel
    let (tx, rx) = mpsc::channel();

    // Spawn a new thread and move the transmitter into it
    thread::spawn(move || {
        let val = String::from("hello");
        // Send the value to the main thread
        tx.send(val).unwrap();
    });

    // Receive the value from the spawned thread
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

In this example, the main thread creates a channel and the spawned thread sends a message back to the main thread. The `recv` method on the main thread's receiver then waits for a message to be sent.

### 3. Shared-State Concurrency:

Rust also supports shared-state concurrency, like using mutexes to access data between threads. However, thanks to Rust's ownership rules, you get additional safety guarantees.

Here's an example of using a mutex in Rust:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // This Arc (Atomic Reference Count) will let multiple threads share ownership of the mutex.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Clone the Arc to increase the reference count
        let counter = Arc::clone(&counter);
        // Spawn new threads
        let handle = thread::spawn(move || {
            // Lock the mutex and update the value inside
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Lock the mutex and print the result
    println!("Result: {}", *counter.lock().unwrap());
}
```

In this example, multiple threads are incrementing a shared counter. The `Arc` type provides shared ownership of the mutex between multiple threads, and the `Mutex` ensures that only one thread can access the data at any time.

### Leveraging Rust's Safety Guarantees:

Rust's concurrency models are designed to prevent common problems found in concurrent programming. Here's how:

1. **Ownership and Borrowing**: Rust's ownership model ensures that there are no data races, as the data can only be owned by one thread or borrowed immutably by multiple threads.

2. **Type Checking**: Rust's type system ensures that you use locks correctly. For instance, you can't accidentally forget to release the lock because the lock guard will be released when it goes out of scope.

3. **No Null Pointers**: Rust's option and result types eliminate null references, ensuring safer access to shared data.

4. **Deadlock Prevention**: While Rust can't prevent deadlocks, it avoids some situations leading to deadlocks, as locks are released automatically when they go out of scope.

By enforcing strict rules at compile time, Rust helps you write more reliable and efficient concurrent programs, preventing entire classes of errors common in concurrent programming in other languages.