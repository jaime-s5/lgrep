use clap::{Arg, ArgGroup, Command};

enum Operation {
    File { name: String },
    Recursive { path: String },
}

fn search_file(name: &str, text_match: &str) {
    let text = "\
    Lorem ipsum dolor sit amet, 
    consectetur adipiscing elit, 
    sed do eiusmod tempor incididunt 
    ut labore et dolore magna aliqua. 
    Ut enim ad minim veniam, quis nostrud 
    exercitation ullamco laboris nisi ut
    aliquip ex ea commodo consequat. 
    Duis aute irure dolor in reprehenderit 
    in voluptate velit esse cillum dolore 
    eu fugiat nulla pariatur. Excepteur sint 
    occaecat cupidatat non proident, sunt in 
    culpa qui officia deserunt mollit anim 
    id est laborum.";

    for (i, line) in text.lines().enumerate() {
        if line.contains(text_match) {
            println!("{}: {}", i, line);
        }
    }
}

fn main() {
    let matches = Command::new("lgrep")
        .author("jaime-s5")
        .version("0.1")
        .about("Searchs for a string in the specified file")
        .arg(
            Arg::new("string")
                .short('s')
                .long("string")
                .takes_value(true)
                .required(true)
                .value_name("STRING")
                .help("String to search for"),
        )
        .group(
            ArgGroup::new("req_flags")
                .required(true)
                .multiple(false)
                .arg("file")
                .arg("recursive"),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .takes_value(true)
                .value_name("DIR")
                .help("Searchs recursively in the specified directory"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .value_name("FILE")
                .help("File to do the search on"),
        )
        .after_help(concat!(
            "Light version of grep that allows to search a ",
            "string in a file, and prints the lines that match."
        ))
        .get_matches();

    // Required arg, so it will always yield Some
    let text_match = matches.value_of("string").unwrap();

    // Either the search is done in one single file or
    // recursively in the specified directory
    let operation = if matches.is_present("file") {
        let name = matches.value_of_t_or_exit("file");
        Operation::File { name }
    } else {
        let path = matches.value_of_t_or_exit("recursive");
        Operation::Recursive { path }
    };

    match operation {
        Operation::File { name } => search_file(&name, text_match),
        Operation::Recursive { path } => todo!(),
    }
}
