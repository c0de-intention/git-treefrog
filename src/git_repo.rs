use std::path::PathBuf;

use git2::Repository;

use crate::{git_path::GitPath, git_worktree::GitWorktree};

#[derive(Debug, Default)]
pub struct GitRepo {
    path: Option<PathBuf>,
}

impl GitRepo {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path }
    }
    fn get_repo(&self) -> Result<Repository, git2::Error> {
        if let Some(path) = &self.path {
            Repository::open(path)
        } else {
            Repository::open_from_env()
        }
    }

    pub fn get_worktrees(&self) -> Result<Vec<GitWorktree>, git2::Error> {
        let repo = self.get_repo()?;
        let worktree_names = repo.worktrees()?;

        let worktrees = worktree_names
            .iter()
            .flatten()
            .flatten()
            .filter_map(|name| {
                let worktree = repo.find_worktree(name).ok()?;
                let sub_repo = Repository::open(worktree.path()).ok()?;
                let path = worktree.path().canonicalize().ok()?;
                let head = &sub_repo.head().ok()?;
                if !head.is_branch() {
                    return None;
                }
                let branch = head.shorthand().ok()?;
                Some(GitWorktree::new(GitPath::new(path), branch.to_string()))
            })
            .collect();
        Ok(worktrees)
    }
}
