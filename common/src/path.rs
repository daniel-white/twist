use std::{
    fs::{create_dir_all, metadata},
    path::{Component, Path, PathBuf},
    rc::Rc,
};

use dirs::home_dir;
use log::debug;

const HOME_DIR_PREFIX: &str = "~";
const HOME_DIR_MAP_NAME: &str = "home";
const HIDDEN_FILE_PREFIX: &str = ".";

pub const FILES_DIR_NAME: &str = "dotfiles";

#[derive(Debug)]
pub struct Paths {
    pub home_dir: PathBuf,
    pub root_dir: PathBuf,
    pub files_dir: PathBuf,
}

#[derive(Debug, PartialEq)]
pub struct FilePathInfo {
    pub full_src_path: PathBuf,
    pub src_path: PathBuf,
    pub full_repo_path: PathBuf,
    pub repo_path: PathBuf,
    pub config_repo_path: PathBuf,
}

impl Paths {
    fn new_with_home_dir<P: AsRef<Path>>(root: P, home_dir: PathBuf) -> Rc<Self> {
        let root_dir = root.as_ref().to_path_buf();
        let files_dir = root_dir.join(FILES_DIR_NAME);

        Rc::new(Paths {
            home_dir,
            root_dir,
            files_dir,
        })
    }

    pub fn new<P: AsRef<Path>>(root: P) -> Rc<Self> {
        Self::new_with_home_dir(root, home_dir().unwrap())
    }

    pub fn resolve_file_paths<P: AsRef<Path>>(&self, p: P) -> Option<FilePathInfo> {
        let full_src_path = p.as_ref().to_path_buf();

        if full_src_path.starts_with(&self.root_dir) {
            debug!("file is in root dir, skipping");
            return None;
        }

        let src_path = self.truncate_home_path(&full_src_path);
        let config_repo_path = self.repo_path(&src_path);
        let repo_path = PathBuf::from(FILES_DIR_NAME).join(&config_repo_path);
        let full_repo_path = self.files_dir.join(&config_repo_path);

        let file_paths = FilePathInfo {
            full_src_path,
            src_path,
            config_repo_path,
            repo_path,
            full_repo_path,
        };

        debug!("resolved file paths: {:?}", file_paths);

        Some(file_paths)
    }

    fn truncate_home_path(&self, p: &Path) -> PathBuf {
        if p.starts_with(&self.home_dir) {
            Path::new(HOME_DIR_PREFIX).join(p.strip_prefix(&self.home_dir).unwrap())
        } else {
            p.to_path_buf()
        }
    }

    fn repo_path(&self, p: &Path) -> PathBuf {
        let mut path = PathBuf::new();

        for p in p.components() {
            match p {
                Component::Normal(c) if c == HOME_DIR_PREFIX => {
                    path.push(HOME_DIR_MAP_NAME);
                }
                Component::Normal(c) => match c.to_str() {
                    Some(c) if c.starts_with(HIDDEN_FILE_PREFIX) => {
                        path.push(c.strip_prefix(HIDDEN_FILE_PREFIX).unwrap());
                    }
                    Some(c) => {
                        path.push(c);
                    }
                    None => {
                        panic!("invalid path component: {:?}", c);
                    }
                },
                _ => {}
            }
        }

        path
    }

    pub fn ensure_parent_dir<P: AsRef<Path>>(p: P) {
        let p = p.as_ref().parent().unwrap();
        debug!("Ensuring directory exists: {:?}", p);
        match metadata(&p) {
            Ok(metadata) if (metadata.is_file()) => panic!("file exists"),
            _ => create_dir_all(&p).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_root_dir() {
        let paths = Paths::new("/home/user/.twist");
        assert_eq!(paths.root_dir, PathBuf::from("/home/user/.twist"));
    }

    #[test]
    fn test_files_dir() {
        let paths = Paths::new(Path::new("/home/user/.twist"));
        assert_eq!(paths.files_dir, Path::new("/home/user/.twist/dotfiles"));
    }

    #[test]
    fn test_resolve_file_paths_no_include_root() {
        let paths = Paths::new("/home/user/.twist");
        assert_eq!(
            paths.resolve_file_paths("/home/user/.twist/dotfiles/file"),
            None
        );
    }

    #[test]
    fn test_resolve_file_paths() {
        let paths = Paths::new_with_home_dir("/home/user/.twist", PathBuf::from("/home/user"));

        assert_eq!(
            paths.resolve_file_paths("/home/user/test"),
            Some(FilePathInfo {
                full_src_path: PathBuf::from("/home/user/test"),
                src_path: PathBuf::from("~/test"),
                config_repo_path: PathBuf::from("home/test"),
                repo_path: PathBuf::from("dotfiles/home/test"),
                full_repo_path: PathBuf::from("/home/user/.twist/dotfiles/home/test"),
            })
        );

        assert_eq!(
            paths.resolve_file_paths("/home/user/.zshrc"),
            Some(FilePathInfo {
                full_src_path: PathBuf::from("/home/user/.zshrc"),
                src_path: PathBuf::from("~/.zshrc"),
                config_repo_path: PathBuf::from("home/zshrc"),
                repo_path: PathBuf::from("dotfiles/home/zshrc"),
                full_repo_path: PathBuf::from("/home/user/.twist/dotfiles/home/zshrc"),
            })
        );

        assert_eq!(
            paths.resolve_file_paths("/home/user/.config/starship.toml"),
            Some(FilePathInfo {
                full_src_path: PathBuf::from("/home/user/.config/starship.toml"),
                src_path: PathBuf::from("~/.config/starship.toml"),
                config_repo_path: PathBuf::from("home/config/starship.toml"),
                repo_path: PathBuf::from("dotfiles/home/config/starship.toml"),
                full_repo_path: PathBuf::from(
                    "/home/user/.twist/dotfiles/home/config/starship.toml"
                ),
            })
        );

        assert_eq!(
            paths.resolve_file_paths("/usr/etc/config.toml"),
            Some(FilePathInfo {
                full_src_path: PathBuf::from("/usr/etc/config.toml"),
                src_path: PathBuf::from("/usr/etc/config.toml"),
                config_repo_path: PathBuf::from("usr/etc/config.toml"),
                repo_path: PathBuf::from("dotfiles/usr/etc/config.toml"),
                full_repo_path: PathBuf::from("/home/user/.twist/dotfiles/usr/etc/config.toml"),
            })
        )
    }
}
