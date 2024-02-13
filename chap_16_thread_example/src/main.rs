use std::{sync::{mpsc, Arc, Mutex}, thread};

fn main() {
    let count: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    let (tx, rx) = mpsc::channel::<String>();
    
    for i in 0..500 {    
        let tx = tx.clone();
        let count1 = Arc::clone(&count);
        let handle = thread::spawn(move || {
            let mut arr = count1.lock().unwrap();
            *arr += 1;
            tx.send(format!("Thread {}: {}", i, arr))
        });

        handles.push(handle);
    }

    for i in 0..500 {
        let tx = tx.clone();
        let count2 = Arc::clone(&count);
        let handle = thread::spawn(move || {
            let mut arr = count2.lock().unwrap();
            *arr += 1;
            tx.send(format!("Thread {}: {}", i, arr))
        });
    
        handles.push(handle);
    }

    for _ in 0..500 {
        let mut arr = count.lock().unwrap();
        *arr += 1;

        tx.send(format!("Main: {}", arr)).unwrap();
    }

    drop(tx);

    for handle in handles {
        match handle.join() {
            Err(err) => println!("{:?}", err),
            _ => {}
        }
    }

    for message in rx {
        println!("{:?}", message);
    }
}
