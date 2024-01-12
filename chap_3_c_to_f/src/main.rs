use std::io;

fn main() {
    println!("Temperature Converter\nTemperature in celcius:");

    let mut celcius_input = String::new();
    
    io::stdin()
        .read_line(&mut celcius_input)
        .expect("Failed to read line");

    let celcius_input: u32 = celcius_input.trim().parse().expect("Entered temperature was not a digit");

    let farenheight = ((celcius_input * 9)/5) + 32;

    println!("{celcius_input}C is {farenheight}F");
}
