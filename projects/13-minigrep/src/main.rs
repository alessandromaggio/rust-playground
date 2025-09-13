use std::env;
use std::fs;
use std::error::Error;
use minigrep::{search, search_case_insensitive};

struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("You must provide exactly two arguments: <query> <filename>");
        }

        let config = Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        };

        Ok(config)
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
