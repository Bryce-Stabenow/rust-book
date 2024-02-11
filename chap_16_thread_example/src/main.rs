use std::{sync::{Arc, Mutex}, thread};

fn main() {
    let count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    
    for _ in 0..100 {    
        let count1 = count.clone();
        thread::spawn(move || {
                let mut arr = count1.lock().unwrap();
                *arr += 1
            });
    }

    for _ in 0..50 {
    let count2 = Arc::clone(&count);
    thread::spawn(move || {
            let mut arr = count2.lock().unwrap();
            *arr += 1;
        });
    }
    
    println!("Count of {}", count.lock().unwrap());
}
