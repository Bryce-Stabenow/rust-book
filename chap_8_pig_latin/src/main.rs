use std::io;
use std::io::Write;

fn main() {
    println!("---------- Ig-pay Atin-lay Onverter-cay ----------");
    let mut input: String = String::new();
    print!("Enter your text: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("ERROR COMPUTER ON FIRE");

    let input = input.trim(); // Remove newline

    println!("Pig Latin: {}", translate(input));
}


fn translate(input: &str) -> String {
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'y'];
    let mut output: String = String::new();

    for word in input.split_whitespace() {
        let first = word.chars().next().unwrap_or(' ').to_ascii_lowercase();

        match vowels.contains(&first) {
            true => output.push_str(&format!("{}-hay ", &word[..])),
            false => output.push_str(&format!("{}-{}ay ", &word[1..], first)),
        }
    }

    String::from(output.trim())
}