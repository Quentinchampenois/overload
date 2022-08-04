use std::fs::File;
use std::io::{BufRead, BufReader};

fn ignored_overloads(filename: &str) -> Vec<String> {
    let file = File::open(filename);
    if let Err(e) = file {
        println!("Error while reading file '{}' : {}", filename, e);
        std::process::exit(1);
    }

    let reader = BufReader::new(file.unwrap());
    reader.lines()
        .map(|l| format!("./{}", l.expect("Could not parse line")))
        .collect::<Vec<String>>()
}

fn main() {
    let ignored_files = ignored_overloads(".overloadignore");
    println!("{:?}", ignored_files);

    std::process::exit(0);
}