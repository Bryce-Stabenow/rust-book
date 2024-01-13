use std::{io::{self, Write}, collections::HashMap};

fn main() {
    println!("Mean, median!\nGive me some numbers, and enter 'crunch' when you're ready to calculate\n");
    let mut numbers: Vec<i32> = get_user_input();

    numbers.sort();

    println!("Median: {}, Mode: {}", get_median(&numbers), get_mode(&numbers));
}


fn get_user_input() -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();

    loop {
        let mut num: String = String::new();
        print!("Number: ");
        io::stdout().flush().unwrap();
    
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line, try again");

        let num: &str = num.trim();
        if num == "crunch" {return vec}

        match num.trim().parse() {
            Ok(num) => vec.push(num),
            Err(_) => {println!("Only whole numbers are accepted"); continue},
        };
    }
}


fn get_median(numbers: &Vec<i32>) -> &i32{
    let i: usize = numbers.len() / 2;
    numbers.get(i).unwrap()
}


fn get_mode(numbers: &Vec<i32>) -> i32{
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut occurances = 0;
    let mut mode: i32 = 0;

    for i in numbers{
        map.entry(*i)
            .and_modify(|num| *num += 1)
            .or_insert(1);
    }

    for (k,v) in map {
        if occurances < v {
            mode = k;
            occurances = v;
        }
    }

    mode
}