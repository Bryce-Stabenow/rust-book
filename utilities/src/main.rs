fn get_drink_by_profession(param: &str) -> &'static str {
    match param.to_lowercase().as_str() {
        "jabroni" => "Patron Tequila",
        "school counselor" => "Anything with Alcohol",
        "programmer" => "Hipster Craft Beer",
        "bike gang member" => "Moonshine",
        "politician" => "Your tax dollars",
        "rapper" => "Cristal",
        _ => "Beer"
    };
}

fn warn_the_sheep(queue: &[&str]) -> String {
    let wolf_location = queue.iter().position(|&x| x == "wolf");
    let length = queue.len();
    
    match wolf_location {
        Some(num) => {
            if num == length - 1{
                "Pls go away and stop eating my sheep".to_string()
            } else {
                format!("Oi! Sheep number {}! You are about to be eaten by a wolf!", length - num - 1)
            }
        },
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

fn build_square(n: i32) -> String{
    (0 .. n)
    .map(|_x| "+".repeat(n as usize))
    .collect::<Vec<String>>().join("\n")
}

/// # Returns the total angle degrees of any shape
/// Since any shape can be divided into a minimum number of triangles
/// we can determine the total degree of the angles by subtracting
/// 2 from the total of sides, and multiplying by 180 deg.
/// Ex. 6 sides on a hexagon, which can be divided into a minimum of 
/// 4 triangles => (6-2) * 180 == 720.
fn angle_finder(sides: usize) -> usize{
    (sides - 2) * 180
}
