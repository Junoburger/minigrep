use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    // let _ = &args.iter().for_each(|x| println!("{}", x));

    if &args.len() > &3 {
        println!(
            "Too many arguments after {1} while searching {0}, try again",
            query, filename
        );
        return;
    }

    println!("Searching for {}", query);
    println!("In file {}", filename);
    let msg = "No match found or something went wrong";
    let contents = fs::read_to_string(filename).expect(&msg);

    println!("With text:\n{}", contents);
}
