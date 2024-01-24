use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let args: Args = Args::build(&args)
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });

    run(args);
}


fn run(args: Args) -> Result<(), Box<dyn Error>>{
    let file_contents: String = fs::read_to_string(args.file_path)?;

    println!("{}", file_contents);

    Ok(())
}


struct Args {
    query: String,
    file_path: String
}


impl Args {
    fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() != 3 {
            return Err("Usage: minigrep <query> <file_path>");
        }

        Ok(Args {
            query: args[1].clone(),
            file_path: args[2].clone()
        })
    }
}
