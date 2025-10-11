use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc}; // mpsc stands for multiple producer, single consumer

fn basic_concurrency() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // This ensures we wait for the spawned thread to finish. Note, this effectively blocks the main thread until the spawned thread is done.
}


fn move_concurrency() {
    let v = vec![1, 2, 3];

    // By default, Rust's closures capture variables by reference
    // With the move keyword, we force the closure to take ownership of v (Rust will infer what are the variables to move)
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn channels() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // We can clone the transmitter to have multiple producers
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap(); // send takes ownership of val
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx1.send(val).unwrap(); // send takes ownership of val
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    // let received = rx.recv().unwrap(); // recv will block the main thread until it receives a value
    // An alternative would be using try_recv which does not block and returns a Result<T, E> immediately
    // println!("Got: {}", received);
}

fn mutex_concurrency() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // lock() will block the thread until it can acquire the lock
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    mutex_concurrency();
    basic_concurrency();
    move_concurrency();
    channels();
}
