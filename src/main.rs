use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = ".overloadignore";

    let file = File::open(filename);
    if let Err(e) = file {
        println!("Error while reading file '{}' : {}", filename, e);
        std::process::exit(1);
    }

    let reader = BufReader::new(file.unwrap());
    let lines = reader.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();
    println!("{:?}", lines);

    std::process::exit(0);
}