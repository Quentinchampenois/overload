mod commit;
mod filesystem;

use std::fs;
use std::process::Command;
use commit::{Commits, Commit};
use filesystem as fss;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    // Retrieve exclude file mentioned in '.overloadignore'
    let excluded = fss::lines_to_vec(".overloadignore");
    let mut overloads : Vec<String> = Vec::new();
    let mut file = fss::find_or_create_file("OVERLOADS.md");
    let mut commits : Commits = Commits {
        commits: vec![]
    };

    // Read files in top-level tree
    let target_dir = match fs::read_dir("./") {
        Ok(file) => file,
        Err(e) => {
            println!("Unexpected error : {:?}", e);
            std::process::exit(1);
        }
    };

    for file in target_dir {
        let unwrap = file.unwrap();
        if unwrap.file_name() == "." || unwrap.file_name() == ".." || unwrap.file_name() == ".overloadignore" {
            continue;
        }
        if excluded.contains(&unwrap.file_name().into_string().unwrap()) {
            continue;
        }
        overloads.push(unwrap.file_name().into_string().unwrap());
    }

    for overload in overloads {
        let target = overload;
        let output = Command::new("git")
        .arg("log")
        .arg("-n 1")
        .arg("--pretty=format:%C(auto)%h :: %as :: %an :: %s")
        .arg("--")
        .arg(&target)
        .output()
        .expect("failed to execute process");

        let commit_msg = std::str::from_utf8(&output.stdout).unwrap();
        let vec: Vec<&str> = commit_msg.split("::").collect();

        // Continue if file isn't in Git history
        if commit_msg == "" {
            continue;
        }

        commits.add(Commit {
            hash: String::from(vec[0].trim()),
            title: String::from(vec[vec.len() - 1].trim()),
            filename: String::from(&target),
        })
    }

    let mut buffer_reader = String::new();
    file.read_to_string(&mut buffer_reader);

    for commit in commits.commits.clone() {
        let mut commit_clone = commit.clone();
        let s_slice: &str = &commit_clone.hash[..];

        if buffer_reader.contains(s_slice) == true {
            continue;
        }

        if let Err(e) = writeln!(file, "{}", commits.display_by_hash(commit_clone.hash.clone())) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    std::process::exit(0);
}