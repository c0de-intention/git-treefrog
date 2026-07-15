use std::path::PathBuf;

use git2::{Repository, RepositoryInitOptions, WorktreeAddOptions};

pub struct Fixture {
    pub path: PathBuf,
    pub repo: Repository,
}

impl Fixture {
    pub fn new(path: &PathBuf) -> Self {
        let mut opts = RepositoryInitOptions::new();
        opts.initial_head("main");
        let repo = Repository::init_opts(path, &opts).unwrap();

        {
            let mut config = repo.config().unwrap();
            config.set_str("user.name", "name").unwrap();
            config.set_str("user.email", "email").unwrap();
            let mut index = repo.index().unwrap();
            let id = index.write_tree().unwrap();

            let tree = repo.find_tree(id).unwrap();
            let sig = repo.signature().unwrap();
            repo.commit(Some("HEAD"), &sig, &sig, "initial\n\nbody", &tree, &[])
                .unwrap();
        }
        Self {
            repo,
            path: path.to_path_buf(),
        }
    }

    pub fn create_worktree(&mut self, name: &str) {
        let opts = WorktreeAddOptions::new();

        let _ = self
            .repo
            .worktree(name, &self.path.join(name), Some(&opts))
            .unwrap();
    }
}
