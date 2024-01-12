use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("--------------------------------\n|     --==GUESS STATION==--    |\n--------------------------------\nCan you guess the number in 5 guesses or less? It's between 1 and 100...\n");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    let mut guess_count: u32 = 5;

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nIt's larger than {guess} ({guess_count} guesses left!)"),
            Ordering::Greater => println!("\nIt's smaller than {guess} ({guess_count} guesses left!)"),
            Ordering::Equal => {
                println!("\nThat's correct! You win!");
                break;
            },
        }

        if guess_count <= 0 {
            println!("\nYou lose :( The secret number was {secret_number}");
            break;
        }

        guess_count -= 1;
    }
}
