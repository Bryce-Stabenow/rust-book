use std::env;
use std::process;

use minigrep::Args;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let args: Args = Args::build(&args)
        .unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1);
        });

    if let Err(e) = minigrep::run(args) {
        println!("{}", e);
        process::exit(1);
    };
}
