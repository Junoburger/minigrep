pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    if args.len() > 3 {
        panic!(
            "Too many arguments after {1} while searching {0}, try again",
            query, filename
        );
    }

    if args.len() < 3 {
        panic!("not enough arguments");
    }

    Config { query, filename }
}
