use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10000 {
            println!("Thread: number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5000 {
        println!("Main: number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
