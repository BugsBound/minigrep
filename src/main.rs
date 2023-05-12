use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = args.get(1).unwrap();
    let filename = &args[2];

    println!("T:{} - F:{}", query, filename);
}
