#[derive(Clone, Debug)]
pub struct Commit {
    pub hash: String,
    pub title: String,
    pub files: Vec<String>
}

impl Commit {
    pub fn add(&mut self, filename: &String) {
        &self.files.push(filename.to_string());
    }
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

    pub fn update(self, hash: String, filename: &String) -> bool {
        for mut commit in self.commits.iter() {
            if commit.hash == hash {
                commit.add(filename);
                return true;
            }
        }

        return false;
    }

    pub fn add(&mut self, commit: Commit) {
        &self.commits.push(commit);
    }
}
