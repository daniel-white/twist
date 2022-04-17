pub mod git;

use anyhow::Result;
use twist_shared::FilePathInfo;

pub trait Repository {
    fn add_files(&self, files: &[FilePathInfo]) -> Result<()>;

    fn commit(&self, message: &str) -> Result<()>;
}
