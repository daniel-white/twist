use std::path::{Path, PathBuf};

use twist_shared::DOTFILES_DIR_NAME;

pub struct Paths {
    pub root_dir: PathBuf,
    pub repo_dir: PathBuf,
}

impl Paths {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        let root_dir = root_dir.as_ref();

        Paths {
            root_dir: root_dir.to_path_buf(),
            repo_dir: root_dir.to_path_buf().join(DOTFILES_DIR_NAME),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_root_dir() {
        let paths = Paths::new(Path::new("/home/user/.twist"));
        assert_eq!(paths.root_dir.to_str().unwrap(), "/home/user/.twist");
    }

    #[test]
    fn test_repo_dir() {
        let paths = Paths::new(Path::new("/home/user/.twist"));
        assert_eq!(
            paths.repo_dir.to_str().unwrap(),
            "/home/user/.twist/dotfiles"
        );
    }
}
