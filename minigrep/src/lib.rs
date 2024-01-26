use std::error::Error;
use std::fs;
use std::env;


pub fn run(args: Args) -> Result<(), Box<dyn Error>>{
    let file_contents: String = fs::read_to_string(args.file_path)?;

    let results: Vec<&str> = match args.case_insensitive{
        true => search_case_insensitive(&args.query, &file_contents),
        false => search(&args.query, &file_contents),
    };

    if results.len() > 0 {
        for line in results{
            println!("{line}");
        };
    } else {
        println!("No matches found");
    }

    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results
}


pub struct Args {
    query: String,
    file_path: String,
    case_insensitive: bool,
}


impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() != 3 {
            return Err("Usage: minigrep <query> <file_path>");
        }

        Ok(Args {
            query: args[1].clone(),
            file_path: args[2].clone(),
            case_insensitive: env::var("IGNORE_CASE").is_ok(),
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}