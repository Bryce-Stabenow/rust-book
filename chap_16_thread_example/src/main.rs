use std::{sync::{Arc, Mutex}, thread};

fn main() {
    let count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..50 {    
        let count1 = count.clone();
        let handle = thread::spawn(move || {
            let mut arr = count1.lock().unwrap();
            *arr += 1;
            println!("Thread 1 - Count: {}", arr);
        });

        handles.push(handle);
    }

    for _ in 0..50 {
        let count2 = Arc::clone(&count);
        let handle = thread::spawn(move || {
            let mut arr = count2.lock().unwrap();
            *arr += 1;
            println!("Thread 2 - Count: {}", arr);
        });
    
        handles.push(handle);
    }

    for _ in 0..50 {
        let mut arr = count.lock().unwrap();
        *arr += 1;

        println!("Main - Count: {}", arr);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
