use std::path::PathBuf;

#[derive(Debug)]
pub struct GitPath(PathBuf);

impl GitPath {
    pub fn new(path_buf: PathBuf) -> Self {
        Self(path_buf)
    }

    pub fn get_short_path(&self) -> String {
        let mut short_path = String::new();
        let mut parts = self.0.iter().rev();
        // let mut parts = self.0.ancestors();
        for _ in 0..3 {
            if let Some(part) = parts.next()
                && let Some(part) = part.to_str()
            {
                short_path = format!("{}/{}", part, short_path,);
            }
        }
        short_path
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
