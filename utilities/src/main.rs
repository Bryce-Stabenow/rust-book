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
    let wolf_location = queue.rev().iter().position(|&x| x == "wolf");
    
    match wolf_location {
        Some(num) => {
            if num == 1{
                "Pls go away and stop eating my sheep".to_string()
            } else {
                format!("Oi! Sheep number {}! You are about to be eaten by a wolf!", num)
            }
        },
        None => "Error".to_string(),
    }
}
