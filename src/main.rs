use std::{env, process};
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Fail with args: {}", err);
        process::exit(1);
    });

    println!("Req:{} - File:{}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something wrong when read file.");

    println!("Text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("need Query and Filename!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
