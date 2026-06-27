use std::path::{Display, PathBuf};

#[derive(Debug)]
pub struct GitPath(PathBuf);

impl GitPath{
    pub fn new(path_buf: PathBuf) -> Self{
        Self(path_buf)
    }
}

impl std::fmt::Display for GitPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self
            .0
            .canonicalize()
            .map_err(|_| std::fmt::Error)?
            .to_str()
            .ok_or(std::fmt::Error)?
            .to_string();
        write!(f, "{}", path)
    }
}
