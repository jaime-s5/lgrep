use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

use clap::{Arg, ArgGroup, Command};
use regex::Regex;

// ANSI escape sequences for highlighting matches
// Adds foreground Green Color with 32 and makes it bold with 1
// The END_COLOR clears the formatting
const START_COLOR: &str = "\x1b[32;1m";
const END_COLOR: &str = "\x1b[0m";

enum Operation {
    File { name: String },
    Recursive { path: String },
}

struct LineDetails {
    line: String,
    number: usize,
}

impl LineDetails {
    fn new() -> Self {
        LineDetails {
            line: String::new(),
            number: 0,
        }
    }

    fn update_details(&mut self, line: String, number: usize) {
        self.line = line;
        self.number = number;
    }
}

/// Searchs string text_match in file and prints matches and
/// number_lines lines before and after
fn search_file(name: &str, text_match: &str, number_lines: usize) {
    let reader = match File::open(name) {
        Ok(file) => BufReader::new(file),
        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        }
    };

    let mut prev_lines: Vec<LineDetails> = Vec::with_capacity(number_lines);
    let mut current_match = LineDetails::new();
    for (i, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("{}", error);
                exit(1);
            }
        };

        // If match, we print stored prev_lines and current
        if line.contains(text_match) {
            for line_detail in &prev_lines {
                println!("{}-{}: {}", name, line_detail.number, line_detail.line);
            }
            prev_lines.clear();

            // In reverse order since inserting the escape characters
            // displaces the string
            let mut colored_line = line.clone();
            for (index, _) in line.rmatch_indices(text_match) {
                colored_line.insert_str(index + text_match.len(), END_COLOR);
                colored_line.insert_str(index, START_COLOR);
            }

            println!("{}-{}:{}", name, i, colored_line);
            current_match.update_details(line, i);

            continue;
        }

        if number_lines == 0 {
            continue;
        }

        // If line is between match and match + number_lines it gets printed
        if !current_match.line.is_empty()
            && i > current_match.number
            && i <= current_match.number + number_lines
        {
            println!("{}-{}: {}", name, i, line);
            continue;
        }

        // Store possible before lines with max number_lines size
        prev_lines.push(LineDetails { line, number: i });
        if prev_lines.len() > number_lines {
            prev_lines.remove(0);
        }
    }
}

// TODO: Add tests
fn main() {
    let reg = Regex::new(r"^(\d{1,2}|[01][0-9][0-9]|2[0-4][0-9]|25[0-5])$").unwrap();
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
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .takes_value(true)
                .value_name("NUMBER")
                .validator_regex(reg, "Only numbers between 0 and 255 are allowed")
                .help("Number of lines to print before and after match"),
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

    // Regex already checks if arg value is a number in range [0, 255]
    let number_lines = match matches.value_of("number") {
        Some(arg) => arg.parse::<usize>().unwrap(),
        None => 0,
    };

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
        Operation::File { name } => search_file(&name, text_match, number_lines),
        #[allow(unused_variables)]
        Operation::Recursive { path } => todo!(),
    }
}
