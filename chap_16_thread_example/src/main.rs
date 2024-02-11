use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2: mpsc::Sender<i32> = tx.clone();

    thread::spawn( move || {
        for i in 1..=10000 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_nanos(1));
        }
    });

    thread::spawn( move || {
        for i in 1..=5000 {
            tx2.send(i).unwrap();
            thread::sleep(Duration::from_nanos(1));
        }
    });

    for message in rx {
        println!("Handler: number {}", message);
    }
}
