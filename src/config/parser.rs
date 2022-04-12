pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    if args.len() > 3 {
        println!(
            "Too many arguments after {1} while searching {0}, try again",
            query, filename
        );
        std::process::exit(1);
    }

    Config { query, filename }
}
