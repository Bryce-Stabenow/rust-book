use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

#[derive(Debug)]
struct LocationData {
    min: f64,
    max: f64,
    total: f64,
    count: u16,
}

impl LocationData {
    fn mean(&self) -> f64 {
        self.total / self.count as f64
    }
}

struct DataPoint {
    location: String,
    temp: f64,
}

fn main() {
    let now = Instant::now();

    let (tx, rx) = mpsc::channel::<DataPoint>();
    let mut handles: Vec<JoinHandle<()>> = vec![];

    let mut results: BTreeMap<String, LocationData> = BTreeMap::new();

    let data: File = File::open("data.txt").unwrap();
    let data: std::io::Lines<BufReader<File>> = BufReader::new(data).lines();

    for line in data {
        let sender = tx.clone();

        let handle = thread::spawn(move || {
            let line: String = line.unwrap();

            println!("{}", line);

            let (location, temp) = line.split_once(';').unwrap();
        
            let point: DataPoint = DataPoint {
                location: location.to_string(),
                temp: temp.parse::<f64>().expect("invalid datapoint"),
            };

            sender.send(point).expect("Cannot parse line");
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    for point in rx.recv() {
    // for point in rx.recv() {
        let l: &mut LocationData = results.entry(point.location).or_insert(LocationData {
            min: 101.0,
            max: -101.0,
            total: 0.0,
            count: 0,
        });
    
        if l.min > point.temp {
            l.min = point.temp;
        }
        if l.max < point.temp {
            l.max = point.temp;
        }
        l.count += 1;
        l.total += point.temp;
    }

    for (name, data) in results {
        println!(
            "{}={:.1}/{:.1}/{:.1}\n",
            name,
            data.min,
            data.mean(),
            data.max
        )
    }

    println!("Runtime: {:.8?}", now.elapsed()); // Current runtime with data.txt - Windows: 4.73093950s, Macbook: 1.13340832s
}
