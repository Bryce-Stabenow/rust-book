use std::env;
use std::process;

use minigrep::Args;

fn main() {    
    let args: Args = Args::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Parsing error: {err}");
            process::exit(1);
        });

    if let Err(e) = minigrep::run(args) {
        eprintln!("Program error: {e}");
        process::exit(1);
    };
}
