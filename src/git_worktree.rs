use crate::git_path::GitPath;

#[derive(Debug)]
pub struct GitWorktree {
    pub name: String,
    pub path: GitPath,
    pub branch: String,
}

impl GitWorktree{
    pub fn new(name: String, path: GitPath, branch: String) -> Self{
        Self { name, path, branch }
    }
}
