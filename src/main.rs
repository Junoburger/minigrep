use std::env;
use std::fs;
extern crate minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::config::parser::parse_config(&args);

    // let _ = &args.iter().for_each(|x| println!("{}", x));

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    let msg = "No match found or something went wrong";
    let contents = fs::read_to_string(config.filename).expect(&msg);

    // fs::write("test.js", config.query).expect(&msg);

    println!("With text:\n{}", contents);
}
