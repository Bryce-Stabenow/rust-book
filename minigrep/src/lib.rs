use std::error::Error;
use std::fs;

pub fn run(args: Args) -> Result<(), Box<dyn Error>>{
    let file_contents: String = fs::read_to_string(args.file_path)?;

    println!("{}", file_contents);

    Ok(())
}


pub struct Args {
    query: String,
    file_path: String
}


impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() != 3 {
            return Err("Usage: minigrep <query> <file_path>");
        }

        Ok(Args {
            query: args[1].clone(),
            file_path: args[2].clone()
        })
    }
}