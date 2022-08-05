mod commit;
mod filesystem;

use std::fs;
use std::process::Command;
use commit::{Commits, Commit};
use filesystem as fss;

fn main() {
    // Retrieve exclude file mentioned in '.overloadignore'
    let excluded = fss::lines_to_vec(".overloadignore");
    let mut overloads : Vec<String> = Vec::new();
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

    println!("{:?}", commits);
    println!("{:?}", commits.commits.sort_by(|a, b| a.partial_cmp(b).unwrap()));
    std::process::exit(0);
}