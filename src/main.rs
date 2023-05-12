use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

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
    fn new(args: &[String]) -> Self {
        Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}
