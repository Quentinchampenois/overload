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

    let mut file = fss::find_or_create_file("OVERLOADS.md");
    let mut commits : Commits = Commits { commits: vec![] };
    let mut overloads : Vec<String> = Vec::new();

    let walker = WalkDir::new(".").into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e, excluded.clone())) {
        let entry = entry.unwrap();
        let filename = entry.file_name().to_string_lossy().parse().unwrap();
        if filename == "." || filename == ".." { continue; }

        overloads.push(filename);
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
        let display_by_hash = commits.display_by_hash(commit_clone.hash.clone());
        let s_display_slice: &str = &display_by_hash[..];

        if buffer_reader.contains(s_slice) { continue; }
        buffer_reader.push_str(s_display_slice);
    }

    if let Err(e) = writeln!(file, "{}", buffer_reader) {
        eprintln!("Couldn't write in file: {}", e);
    }

    std::process::exit(0);
}