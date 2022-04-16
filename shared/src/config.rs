use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::PathBuf,
};

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    #[serde(default)]
    dirs: Dirs,
    #[serde(default)]
    files: Files,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Files(BTreeMap<PathBuf, PathBuf>);

#[derive(Serialize, Deserialize, Debug, Default)]
struct Dirs(BTreeSet<PathBuf>);

pub fn test_write(p: &PathBuf) -> Result<()> {
    let mut test = Config {
        files: Files(BTreeMap::new()),
        dirs: Dirs(BTreeSet::new()),
    };

    test.files
        .0
        .insert(PathBuf::from("x"), PathBuf::from("/home/user/dir/x"));

    test.dirs.0.insert(PathBuf::from("/home/user/dir"));

    let content = toml::to_string_pretty(&test)?;
    fs::write(p, content).unwrap();
    Ok(())
}
