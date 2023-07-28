use std::{
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::Result;
use toml::{from_str as from_toml_str, to_string_pretty as to_toml_string};

use super::{ConfigData, ConfigFilePersistence};

const TOML_CONFIG_FILE_NAME: &str = "config.toml";

pub(super) struct TomlConfigFilePersistence;

impl ConfigFilePersistence for TomlConfigFilePersistence {
    fn file_name() -> PathBuf {
        PathBuf::from(TOML_CONFIG_FILE_NAME)
    }

    fn read(reader: &mut impl Read) -> Result<ConfigData> {
        let mut toml = String::new();
        reader.read_to_string(&mut toml)?;
        Ok(from_toml_str(&toml)?)
    }

    fn write(data: &ConfigData, writer: &mut impl Write) -> Result<()> {
        let toml = to_toml_string(data)?;
        write!(writer, "{}", toml)?;
        Ok(())
    }
}
