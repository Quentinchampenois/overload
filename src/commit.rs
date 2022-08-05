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

    pub fn add(&mut self, commit: Commit) {
        &self.commits.push(commit);
    }
}
