use crate::git_path::GitPath;

#[derive(Debug)]
pub struct GitWorktree {
    pub path: GitPath,
    pub branch: String,
}

impl GitWorktree {
    pub fn new(path: GitPath, branch: String) -> Self {
        Self { path, branch }
    }
}
