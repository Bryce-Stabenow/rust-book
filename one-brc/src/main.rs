use std::collections::HashMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    struct LocationData {
        min: f64,
        max: f64,
        total: f64,
        count: f64,
    }

    impl LocationData {
        fn mean (&self) -> f64 {
            self.total / self.count
        }
    }

    struct DataPoint {
        location: String,
        temp: f64,
    }

    let mut results: HashMap<String, LocationData> = HashMap::new();

    let data: File = File::open("data.txt").unwrap();
    let data: std::io::Lines<BufReader<File>> = BufReader::new(data).lines();

    for line in data {
        let line: String = line.unwrap();
        let (location, temp) = line.split_once(';').unwrap();

        let point: DataPoint = DataPoint{
            location: location.to_string(),
            temp: temp.parse::<f64>().expect("invalid datapoint"),
        };

        let l: &mut LocationData = results.entry(point.location)
            .or_insert(LocationData{min: 101.0, max: -101.0, total: 0.0, count: 0.0});


        if l.min > point.temp {
            l.min = point.temp;
        }
        if l.max < point.temp {
            l.max = point.temp;
        }
        l.count += 1.0;
        l.total += point.temp;
    }

    for (name, data) in results {
        println!("{}={}/{}/{}\n", name, data.min, data.mean(), data.max)
    }
}
