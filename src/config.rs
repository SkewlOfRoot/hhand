use anyhow::Ok;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use crate::bookmarks::Browser;

const CONFIG_PATH: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub browser: Browser,
}

pub fn load() -> anyhow::Result<Config> {
    let path = Path::new(CONFIG_PATH);
    Ok(load_or_default(path)?)
}

fn load_or_default(path: &Path) -> anyhow::Result<Config> {
    if path.exists() {
        Ok(load_from_path(path)?)
    } else {
        Ok(Config::default())
    }
}

fn load_from_path(path: &Path) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    println!("Content: {}", &content);
    let config = toml::from_str(content.as_str())?;
    Ok(config)
}

pub fn save(config: &Config) -> anyhow::Result<()> {
    let config_path = Path::new(CONFIG_PATH);
    save_to_path(config, config_path)?;
    Ok(())
}

fn save_to_path(config: &Config, path: &Path) -> anyhow::Result<()> {
    let str_content = toml::to_string(&config)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(str_content.as_bytes())?;
    Ok(())
}

impl Default for Config {
    fn default() -> Self {
        Config {
            browser: Browser::Firefox,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    const TEST_CONFIG_PATH: &str = "test_config.toml";

    fn cleanup_test_config() {
        let _ = fs::remove_file(TEST_CONFIG_PATH);
    }

    #[test]
    fn test_save_to_path() {
        cleanup_test_config();
        let path = Path::new(TEST_CONFIG_PATH);
        let config = Config::default();

        let save_res = save_to_path(&config, path);
        assert!(save_res.is_ok());

        let load_res = load_from_path(path);
        assert!(load_res.is_ok());

        let loaded_config = load_res.unwrap();
        assert_eq!(&config.browser, &loaded_config.browser);
    }
}
