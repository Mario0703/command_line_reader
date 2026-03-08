use std::{env, fs};
use std::process;
use std::error::Error;
use command_line_reader::{search, search_case_insensitive};


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }


}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

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

// using clone is ok on small datatypes, 

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
// for things like CLI arguments, filenames, queries, config values, cloning is considered normal and idiomatic.
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

