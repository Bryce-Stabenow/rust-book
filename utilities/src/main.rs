fn get_drink_by_profession(param: &str) -> &'static str {
    match param.to_lowercase().as_str() {
        "jabroni" => "Patron Tequila",
        "school counselor" => "Anything with Alcohol",
        "programmer" => "Hipster Craft Beer",
        "bike gang member" => "Moonshine",
        "politician" => "Your tax dollars",
        "rapper" => "Cristal",
        _ => "Beer",
    };
}

fn warn_the_sheep(queue: &[&str]) -> String {
    let wolf_location = queue.iter().position(|&x| x == "wolf");
    let length = queue.len();

    match wolf_location {
        Some(num) => {
            if num == length - 1 {
                "Pls go away and stop eating my sheep".to_string()
            } else {
                format!(
                    "Oi! Sheep number {}! You are about to be eaten by a wolf!",
                    length - num - 1
                )
            }
        }
        None => "Error".to_string(),
    }
}

fn separate_even_and_odd_in_vec(xs: &[i16]) -> Vec<i16> {
    let mut even: Vec<i16> = Vec::new();
    let mut odd: Vec<i16> = Vec::new();

    for num in xs {
        match num % 2 {
            0 => even.push(num.clone()),
            _ => odd.push(num.clone()),
        }
    }

    even.sort();

    odd.sort();
    odd.reverse();

    even.append(&mut odd);
    even.dedup();

    return even;
}

fn build_square(n: i32) -> String {
    (0..n)
        .map(|_x| "+".repeat(n as usize))
        .collect::<Vec<String>>()
        .join("\n")
}

/// # Returns the total angle degrees of any shape
/// Since any shape can be divided into a minimum number of triangles
/// we can determine the total degree of the angles by subtracting
/// 2 from the total of sides, and multiplying by 180 deg.
/// Ex. 6 sides on a hexagon, which can be divided into a minimum of
/// 4 triangles => (6-2) * 180 == 720.
fn angle_finder(sides: usize) -> usize {
    (sides - 2) * 180
}

fn largest(n: usize, xs: &[i32]) -> Vec<i32> {
    let mut answer: Vec<i32> = vec![];
    let mut new_vec = xs.to_owned();
    new_vec.sort();
    new_vec.reverse();

    let mut i: usize = n;

    while i > 0 {
        i = i - 1;
        answer.push(new_vec[i]);
    }

    answer
}

struct Block {
    width: u32,
    length: u32,
    height: u32,
}

impl Block {
    fn new(arr: &[u32]) -> Self {
        Block {
            width: arr[0],
            length: arr[1],
            height: arr[2],
        }
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_length(&self) -> u32 {
        self.length
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn get_volume(&self) -> u32 {
        self.height * self.length * self.width
    }

    fn get_surface_area(&self) -> u32 {
        (2 * self.length * self.height)
            + (2 * self.length * self.width)
            + (2 * self.height * self.width)
    }
}

fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
    let mut v: Vec<i32> = arr.iter().flatten().copied().collect::<Vec<i32>>();
    v.sort();
    v
}

fn part_list(v: Vec<&str>) -> String {
    let mut answer = String::from("");
    let last_idx = v.len() - 1;

    for limit in 0..last_idx {
        let mut substring = String::from("(");

        for num in 0..v.len() {
            substring.push_str(v[num]);

            if num == limit {
                substring.push_str(", ")
            } else if num < last_idx {
                substring.push(' ');
            }
        }

        substring.push(')');
        answer.push_str(&substring);
    }

    answer
}

// Sums arrays of abs(n) + 1 sequence
fn sum_of_n(n: i32) -> Vec<i32> {
    let mut answer = vec![0];
    let limit = n.abs() + 1;
    let mut total = 0;

    for num in 1..limit {
        match n < 0 {
            true => total -= num,
            false => total += num,
        }

        answer.push(total);
    }

    answer
}

use std::collections::HashMap;

fn to_leet_speak(s: &str) -> String {
    let leet_map = get_leet_map();

    s.chars()
        .map(|c| leet_map.get(&c).unwrap_or(&' '))
        .collect::<String>()
}

fn get_leet_map() -> HashMap<char, char> {
    let mut leet_map: HashMap<char, char> = HashMap::new();

    leet_map.insert('A', '@');
    leet_map.insert('B', '8');
    leet_map.insert('C', '(');
    leet_map.insert('D', 'D');
    leet_map.insert('E', '3');
    leet_map.insert('F', 'F');
    leet_map.insert('G', '6');
    leet_map.insert('H', '#');
    leet_map.insert('I', '!');
    leet_map.insert('J', 'J');
    leet_map.insert('K', 'K');
    leet_map.insert('L', '1');
    leet_map.insert('M', 'M');
    leet_map.insert('N', 'N');
    leet_map.insert('O', '0');
    leet_map.insert('P', 'P');
    leet_map.insert('Q', 'Q');
    leet_map.insert('R', 'R');
    leet_map.insert('S', '$');
    leet_map.insert('T', '7');
    leet_map.insert('U', 'U');
    leet_map.insert('V', 'V');
    leet_map.insert('W', 'W');
    leet_map.insert('X', 'X');
    leet_map.insert('Y', 'Y');
    leet_map.insert('Z', '2');

    leet_map
}

// Gets the sum of words where a=1 and z=26
fn words_to_marks(s: &str) -> u32 {
    s.chars().map(|c| c as u32 - 96).sum()
}
