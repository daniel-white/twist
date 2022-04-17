use std::{
    fs::{create_dir_all, metadata},
    path::{Component, Path, PathBuf},
    rc::Rc,
};

use dirs::home_dir;
use log::debug;
use twist_shared::FILES_DIR_NAME;

const HOME_DIR_PREFIX: &str = "~";
const HOME_DIR_MAP_NAME: &str = "home";
const HIDDEN_FILE_PREFIX: &str = ".";

pub struct Paths {
    pub home_dir: PathBuf,
    pub root_dir: PathBuf,
    pub files_dir: PathBuf,
}

impl Paths {
    pub fn new<P: AsRef<Path>>(p: P) -> Rc<Self> {
        let root_dir = p.as_ref().to_path_buf();
        let files_dir = root_dir.join(FILES_DIR_NAME);

        Rc::new(Paths {
            home_dir: home_dir().unwrap(),
            root_dir,
            files_dir,
        })
    }

    pub fn file_path<P: AsRef<Path>>(&self, p: P) -> Option<PathBuf> {
        let p = Path::new(p.as_ref());

        if p.starts_with(&self.root_dir) {
            return None;
        }

        let p = if p.starts_with(&self.home_dir) {
            Path::new(HOME_DIR_PREFIX).join(p.strip_prefix(&self.home_dir).unwrap())
        } else {
            p.to_path_buf()
        };

        let mut path = self.files_dir.clone();
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

        Some(path)
    }

    pub fn ensure_parent_dir<P: AsRef<Path>>(p: P) {
        let p = p.as_ref().parent().unwrap();
        debug!("ensure_parent_dir: {:?}", p);
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
        let paths = Paths::new(Path::new("/home/user/.twist"));
        assert_eq!(paths.root_dir, Path::new("/home/user/.twist"));
    }

    #[test]
    fn test_files_dir() {
        let paths = Paths::new(Path::new("/home/user/.twist"));
        assert_eq!(paths.files_dir, Path::new("/home/user/.twist/dotfiles"));
    }

    #[test]
    fn test_simple_home_file_path() {
        let paths = Paths::new(Path::new("/home/user/.twist"));
        assert_eq!(
            paths.file_path("~/.zshrc"),
            Some(PathBuf::from("/home/user/.twist/dotfiles/home/zshrc"))
        );

        assert_eq!(
            paths.file_path("~/starship/config.toml"),
            Some(PathBuf::from(
                "/home/user/.twist/dotfiles/home/starship/config.toml"
            ))
        );
    }

    #[test]
    fn test_simple_file_path() {
        let paths = Paths::new(Path::new("/home/user/.twist"));
        assert_eq!(
            paths.file_path("/.zshrc"),
            Some(PathBuf::from("/home/user/.twist/dotfiles/zshrc"))
        );

        assert_eq!(
            paths.file_path("/usr/etc/config.toml"),
            Some(PathBuf::from(
                "/home/user/.twist/dotfiles/usr/etc/config.toml"
            ))
        );
    }

    #[test]
    fn test_avoid_self_referential_paths() {
        let paths = Paths::new(Path::new("/home/user/.twist"));
        assert_eq!(paths.file_path("/home/user/.twist/config.toml"), None);
    }
}
