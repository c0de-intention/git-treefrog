use std::fs::{create_dir, remove_dir_all};
use std::path::PathBuf;

pub struct TempDirectory {
    pub path: PathBuf,
}
impl TempDirectory {
    pub fn new(path: PathBuf) -> color_eyre::Result<Self> {
        create_dir("tmp/repo").unwrap();
        Ok(Self { path })
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        remove_dir_all(self.path.clone()).unwrap();
    }
}
