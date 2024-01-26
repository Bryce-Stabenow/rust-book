use std::env;
use std::process;

use minigrep::Args;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let args: Args = Args::build(&args)
        .unwrap_or_else(|err| {
            eprintln!("Parsing error: {err}");
            process::exit(1);
        });

    if let Err(e) = minigrep::run(args) {
        eprintln!("Program error: {e}");
        process::exit(1);
    };
}
