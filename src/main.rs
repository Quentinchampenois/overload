use std::fs;

fn main() {
    let filename = ".overloadignore";

    let contents = fs::read_to_string(filename);
    match contents {
        Ok(_) => {
            println!("File found !");
        },
        Err(e) => {
            println!("Error while reading file '{}' : {}", filename, e);
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}