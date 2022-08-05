use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};

pub fn lines_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename);
    if let Err(e) = file {
        println!("Error while reading file '{}' : {}", filename, e);
        std::process::exit(1);
    }

    let reader = BufReader::new(file.unwrap());
    reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>()
}

pub fn find_or_create_file(filename: &str) -> File {
    std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("OVERLOADS.md").unwrap()
}