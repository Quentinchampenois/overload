#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Commit {
    pub hash: String,
    pub title: String,
    pub path: String
}

impl Commit {
    pub fn format(&self) -> String {
        format!(" * __{} - {}__

    ` {} `
", self.hash, self.title, self.path).to_string()
    }
}
