use std::fs;

fn main() {
    let filename = ".overloadignore";

    let contents = fs::read_to_string(filename);
    match contents {
        Err(e) => {
            println!("Error while reading file '{}' : {}", filename, e);
            std::process::exit(1);
        }
        _ => {}
    }

    println!("{}", contents.unwrap());
    std::process::exit(0);
}