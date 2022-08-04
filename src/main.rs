use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = ".overloadignore";

    let file = File::open(filename);
    match file {
        Err(e) => {
            println!("Error while reading file '{}' : {}", filename, e);
            std::process::exit(1);
        }
        _ => {}
    }

    let reader = BufReader::new(file.unwrap());
    let lines = reader.lines().map(|l| l.expect("Could not parse line")).collect::<Vec<String>>();
    println!("{:?}", lines);

    std::process::exit(0);
}