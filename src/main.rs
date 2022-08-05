mod commit;
mod filesystem;

use std::fs;
use std::process::Command;
use commit::{Commits, Commit};
use filesystem as fss;
use std::fs::OpenOptions;
use std::io::prelude::*;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry, excluded: Vec<String>) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| excluded.contains(&String::from(s)))
        .unwrap_or(false)
}

fn main() {
    // Retrieve exclude file mentioned in '.overloadignore'
    let mut excluded = fss::lines_to_vec(".overloadignore");
    excluded.append(&mut vec![
        String::from("."),
        String::from(".."),
        String::from(".git"),
        String::from(".idea"),
        String::from("OVERLOADS.md"),
        String::from("packages"),
        String::from("public"),
        String::from("tmp"),
        String::from("vendor"),
        String::from("docs"),
        String::from("deploy/providers"),
    ]);

    let mut overloads : Vec<String> = Vec::new();
    let mut file = fss::find_or_create_file("OVERLOADS.md");
    let mut commits : Commits = Commits {
        commits: vec![]
    };

    let walker = WalkDir::new(".").into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e, excluded.clone())) {
        let entry = entry.unwrap();

        overloads.push(entry.file_name().to_string_lossy().parse().unwrap());
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
        if commit_msg == "" { continue; }

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

        if buffer_reader.contains(s_slice) {
            continue;
        }

        if let Err(e) = writeln!(file, "{}", commits.display_by_hash(commit_clone.hash.clone())) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    std::process::exit(0);
}