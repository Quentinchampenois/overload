use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

#[derive(Clone, Debug)]
struct Commit {
    hash: String,
    title: String,
    files: Vec<String>
}
impl Commit {
    fn add(&mut self, filename: &String) {
        &self.files.push(filename.to_string());
    }
}

#[derive(Debug)]
struct Commits {
    commits: Vec<Commit>
}
impl Commits {
    fn find_by(&self, hash: String) -> Option<Commit> {
        for mut commit in self.commits.iter() {
            if commit.hash == hash {
                return Some(commit.clone());
            }
        }
        None
    }

    fn update(self, hash: String, filename: &String) -> bool {
        for mut commit in self.commits.iter() {
            if commit.hash == hash {
                commit.add(filename);
                return true;
            }
        }

        return false;
    }

    fn add(&mut self, commit: Commit) {
        &self.commits.push(commit);
    }
}

fn ignored_overloads(filename: &str) -> Vec<String> {
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

fn main() {
    let ignored_files = ignored_overloads(".overloadignore");
    let mut overloads : Vec<String> = Vec::new();
    println!("{:?}", ignored_files);

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
        if ignored_files.contains(&unwrap.file_name().into_string().unwrap()) {
            continue;
        }
        overloads.push(unwrap.file_name().into_string().unwrap());
    }

    println!("{:?}", overloads);

    let mut commits : Commits = Commits {
        commits: vec![]
    };

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

        let mut found = false;

        commits.update(String::from(vec[0]), &target);

        if !found {
            commits.add(Commit {
                hash: String::from(vec[0]),
                title: String::from(vec[vec.len() -1]),
                files: vec![String::from(&target)]
            });
        }


        let title = format!("{:?}
        * {}", commit_msg, &target);
        println!("{}", title);
    }

    println!("{:?}", commits);
    std::process::exit(0);
}