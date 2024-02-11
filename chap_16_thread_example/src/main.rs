use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2: mpsc::Sender<i32> = tx.clone();

    thread::spawn( move || {
        for i in 1..=1000 {
            tx.send(i).unwrap();
        }
    });

    thread::spawn( move || {
        for i in 1..=500 {
            tx2.send(i).unwrap();
        }
    });

    for message in rx {
        println!("Handler: number {}", message);
    }
}
