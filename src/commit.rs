#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Commit {
    pub hash: String,
    pub title: String,
    pub filename: String
}

pub struct CommitGroup {
    pub commit: Commit,
    pub files: Vec<String>
}

#[derive(Debug)]
pub struct Commits {
    pub commits: Vec<Commit>
}

impl Commits {
    pub fn find_by(&self, hash: String) -> Option<Commit> {
        for mut commit in self.commits.iter() {
            if commit.hash == hash {
                return Some(commit.clone());
            }
        }
        None
    }

    pub fn display_by_hash(&self, hash: String) -> String {
        let mut group: Vec<Commit> = vec![];
        for commit in &self.commits {
            if commit.hash == hash {
                group.push(commit.clone());
            }
        }

        return self.format_display(group);
    }

    pub fn format_display(&self, commits: Vec<Commit>) -> String {
        let mut result : String = format!("{} - {}", commits[0].hash, commits[0].title).to_string();
        for commit in commits {
            let mut format = format!("\\
            * {}", commit.filename);
            let s_slice: &str = &format[..];
            result.push_str(s_slice);
        }

        return result.to_string();
    }

    pub fn add(&mut self, commit: Commit) {
        &self.commits.push(commit);
    }
}
