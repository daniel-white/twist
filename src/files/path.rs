use log::debug;
use std::fs::{create_dir_all, metadata};
use std::path::{Component, Path, PathBuf};

use anyhow::Result;
use dirs::home_dir;
use thiserror::Error;

const ROOT_DIR_NAME: &str = ".twist";
const FILES_DIR_NAME: &str = "dotfiles";
const HOME_DIR_PREFIX: &str = "~";
const HOME_DIR_MAP_NAME: &str = "home";
const HIDDEN_FILE_PREFIX: &str = ".";

#[derive(Debug, Error)]
enum RootDirError {
    #[error("could not find home directory to use as root directory")]
    HomeDirNotFound,
    #[error("the requested root directory is not a directory: {0}")]
    NotADirectory(PathBuf),
}

pub fn root_dir(root_dir_override: Option<PathBuf>) -> Result<PathBuf> {
    let root_dir = root_dir_override
        .or_else(|| home_dir().map(|home_dir| Path::new(&home_dir).join(ROOT_DIR_NAME)));

    match root_dir {
        Some(root_dir) => match metadata(&root_dir) {
            Err(_) => Ok(root_dir),
            Ok(metadata) if metadata.is_dir() => Ok(root_dir),
            _ => Err(RootDirError::NotADirectory(root_dir).into()),
        },
        None => Err(RootDirError::HomeDirNotFound.into()),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paths {
    pub home_dir: PathBuf,
    pub root_dir: PathBuf,
    pub files_dir: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FilePathInfo {
    pub full_src_path: PathBuf,
    pub src_path: PathBuf,
    pub full_repo_path: PathBuf,
    pub repo_path: PathBuf,
    pub config_repo_path: PathBuf,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DirPathInfo {
    pub full_src_path: PathBuf,
    pub src_path: PathBuf,
    pub repo_path: PathBuf,
    pub full_parent_repo_path: PathBuf,
    pub full_repo_path: PathBuf,
    pub config_repo_path: PathBuf,
}

#[derive(Error, Debug)]
pub enum PathsError {
    #[error("The existing path {0} is not a directory")]
    ExistingPathIsNotADirectory(PathBuf),
    #[error("Unable to create directory {0}")]
    UnableToCreateDirectory(PathBuf),
}

impl Paths {
    fn new_with_home_dir<P: AsRef<Path>>(root: P, home_dir: PathBuf) -> Self {
        let root_dir = root.as_ref().to_path_buf();
        let files_dir = root_dir.join(FILES_DIR_NAME);

        Paths {
            home_dir,
            root_dir,
            files_dir,
        }
    }

    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        Self::new_with_home_dir(root, home_dir().unwrap())
    }

    pub fn resolve_paths(&self, paths: &[PathBuf]) -> (Vec<FilePathInfo>, Vec<DirPathInfo>) {
        let mut files = vec![];
        let mut dirs = vec![];

        debug!("resolving paths: {:?}", paths);

        for p in paths {
            match metadata(p) {
                Ok(m) if m.is_file() => {
                    if let Some(p) = self.resolve_file_paths(p) {
                        files.push(p)
                    }
                }
                Ok(m) if m.is_dir() => {
                    if let Some(p) = self.resolve_dir_paths(p) {
                        dirs.push(p)
                    }
                }
                _ => continue,
            }
        }

        debug!("resolved {} files and {} dirs", files.len(), dirs.len());

        (files, dirs)
    }

    pub fn root_dir(&self) -> &Path {
        &self.root_dir
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

    pub fn resolve_file_paths_from_config_paths<P: AsRef<Path>>(
        &self,
        src_path: P,
        config_repo_path: P,
    ) -> FilePathInfo {
        let src_path = src_path.as_ref();
        let config_repo_path = config_repo_path.as_ref();

        let full_src_path = if src_path.starts_with(HOME_DIR_PREFIX) {
            self.home_dir
                .join(src_path.strip_prefix(HOME_DIR_PREFIX).unwrap())
        } else {
            src_path.to_path_buf()
        };

        FilePathInfo {
            full_src_path,
            src_path: src_path.to_path_buf(),
            config_repo_path: config_repo_path.to_path_buf(),
            repo_path: PathBuf::from(FILES_DIR_NAME).join(config_repo_path),
            full_repo_path: self.files_dir.join(&config_repo_path),
        }
    }

    pub fn resolve_dir_paths<P: AsRef<Path>>(&self, p: P) -> Option<DirPathInfo> {
        let full_src_path = p.as_ref().to_path_buf();

        if full_src_path.starts_with(&self.root_dir) {
            debug!("dir is in root dir, skipping");
            return None;
        }

        let src_path = self.truncate_home_path(&full_src_path);
        let config_repo_path = self.repo_path(&src_path);
        let repo_path = PathBuf::from(FILES_DIR_NAME).join(&config_repo_path);
        let full_repo_path = self.files_dir.join(&config_repo_path);
        eprintln!("{:?}", full_repo_path);
        let mut full_parent_repo_path = full_repo_path.clone();
        full_parent_repo_path.pop();

        let dir_paths = DirPathInfo {
            full_src_path,
            src_path,
            config_repo_path,
            repo_path,
            full_repo_path,
            full_parent_repo_path,
        };

        debug!("resolved dir paths: {:?}", dir_paths);

        Some(dir_paths)
    }

    pub fn resolve_dir_paths_from_config_paths<P: AsRef<Path>>(
        &self,
        src_path: P,
        config_repo_path: P,
    ) -> DirPathInfo {
        let src_path = src_path.as_ref();
        let config_repo_path = config_repo_path.as_ref();
        let full_repo_path = self.files_dir.join(&config_repo_path);

        let full_src_path = if src_path.starts_with(HOME_DIR_PREFIX) {
            self.home_dir
                .join(src_path.strip_prefix(HOME_DIR_PREFIX).unwrap())
        } else {
            src_path.to_path_buf()
        };

        DirPathInfo {
            full_src_path,
            src_path: src_path.to_path_buf(),
            repo_path: PathBuf::from(FILES_DIR_NAME).join(config_repo_path),
            full_parent_repo_path: full_repo_path.parent().unwrap().to_path_buf(),
            full_repo_path,
            config_repo_path: config_repo_path.to_path_buf(),
        }
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

    pub fn ensure_parent_dir<P: AsRef<Path>>(p: P) -> Result<()> {
        let p = p.as_ref().parent().unwrap();
        debug!("Ensuring directory exists: {:?}", p);
        match metadata(&p) {
            Ok(metadata) if metadata.is_dir() => {
                debug!("parent path is directory");
                Ok(())
            }
            Ok(_) => Err(PathsError::ExistingPathIsNotADirectory(p.to_path_buf()).into()),
            _ => create_dir_all(&p)
                .map(|_| debug!("created directory"))
                .map_err(|_| PathsError::UnableToCreateDirectory(p.to_path_buf()).into()),
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

    #[test]
    pub fn test_resolve_file_paths_from_config_paths() {
        let paths = Paths::new_with_home_dir("/home/user/.twist", PathBuf::from("/home/user"));

        assert_eq!(
            paths.resolve_file_paths_from_config_paths(
                "~/.config/starship.toml",
                "home/config/starship.toml"
            ),
            FilePathInfo {
                full_src_path: PathBuf::from("/home/user/.config/starship.toml"),
                src_path: PathBuf::from("~/.config/starship.toml"),
                config_repo_path: PathBuf::from("home/config/starship.toml"),
                repo_path: PathBuf::from("dotfiles/home/config/starship.toml"),
                full_repo_path: PathBuf::from(
                    "/home/user/.twist/dotfiles/home/config/starship.toml"
                ),
            }
        );

        assert_eq!(
            paths.resolve_file_paths_from_config_paths(
                "/etc/nginx/nginx.conf",
                "etc/nginx/nginx.conf"
            ),
            FilePathInfo {
                full_src_path: PathBuf::from("/etc/nginx/nginx.conf"),
                src_path: PathBuf::from("/etc/nginx/nginx.conf"),
                config_repo_path: PathBuf::from("etc/nginx/nginx.conf"),
                repo_path: PathBuf::from("dotfiles/etc/nginx/nginx.conf"),
                full_repo_path: PathBuf::from("/home/user/.twist/dotfiles/etc/nginx/nginx.conf"),
            }
        );
    }

    #[test]
    pub fn test_resolve_dir_paths() {
        let paths = Paths::new_with_home_dir("/home/user/.twist", PathBuf::from("/home/user"));

        assert_eq!(
            paths.resolve_dir_paths("/home/user/test"),
            Some(DirPathInfo {
                full_src_path: PathBuf::from("/home/user/test"),
                src_path: PathBuf::from("~/test"),
                config_repo_path: PathBuf::from("home/test"),
                repo_path: PathBuf::from("dotfiles/home/test"),
                full_repo_path: PathBuf::from("/home/user/.twist/dotfiles/home/test"),
                full_parent_repo_path: PathBuf::from("/home/user/.twist/dotfiles/home"),
            })
        );

        assert_eq!(
            paths.resolve_dir_paths("/etc/nginx"),
            Some(DirPathInfo {
                full_src_path: PathBuf::from("/etc/nginx"),
                src_path: PathBuf::from("/etc/nginx"),
                config_repo_path: PathBuf::from("etc/nginx"),
                repo_path: PathBuf::from("dotfiles/etc/nginx"),
                full_repo_path: PathBuf::from("/home/user/.twist/dotfiles/etc/nginx"),
                full_parent_repo_path: PathBuf::from("/home/user/.twist/dotfiles/etc"),
            })
        );

        assert_eq!(
            paths.resolve_dir_paths("/home/user/.ssh"),
            Some(DirPathInfo {
                full_src_path: PathBuf::from("/home/user/.ssh"),
                src_path: PathBuf::from("~/.ssh"),
                config_repo_path: PathBuf::from("home/ssh"),
                repo_path: PathBuf::from("dotfiles/home/ssh"),
                full_repo_path: PathBuf::from("/home/user/.twist/dotfiles/home/ssh"),
                full_parent_repo_path: PathBuf::from("/home/user/.twist/dotfiles/home"),
            })
        );
    }

    #[test]
    pub fn test_resolve_dir_paths_from_config_paths() {
        let paths = Paths::new_with_home_dir("/home/user/.twist", PathBuf::from("/home/user"));

        assert_eq!(
            paths.resolve_dir_paths_from_config_paths("~/test", "home/test"),
            DirPathInfo {
                full_src_path: PathBuf::from("/home/user/test"),
                src_path: PathBuf::from("~/test"),
                config_repo_path: PathBuf::from("home/test"),
                repo_path: PathBuf::from("dotfiles/home/test"),
                full_repo_path: PathBuf::from("/home/user/.twist/dotfiles/home/test"),
                full_parent_repo_path: PathBuf::from("/home/user/.twist/dotfiles/home"),
            }
        );

        assert_eq!(
            paths.resolve_dir_paths_from_config_paths("/etc/nginx", "etc/nginx"),
            DirPathInfo {
                full_src_path: PathBuf::from("/etc/nginx"),
                src_path: PathBuf::from("/etc/nginx"),
                config_repo_path: PathBuf::from("etc/nginx"),
                repo_path: PathBuf::from("dotfiles/etc/nginx"),
                full_repo_path: PathBuf::from("/home/user/.twist/dotfiles/etc/nginx"),
                full_parent_repo_path: PathBuf::from("/home/user/.twist/dotfiles/etc"),
            }
        );

        assert_eq!(
            paths.resolve_dir_paths_from_config_paths("~/.ssh", "home/ssh"),
            DirPathInfo {
                full_src_path: PathBuf::from("/home/user/.ssh"),
                src_path: PathBuf::from("~/.ssh"),
                config_repo_path: PathBuf::from("home/ssh"),
                repo_path: PathBuf::from("dotfiles/home/ssh"),
                full_repo_path: PathBuf::from("/home/user/.twist/dotfiles/home/ssh"),
                full_parent_repo_path: PathBuf::from("/home/user/.twist/dotfiles/home"),
            }
        );
    }
}
