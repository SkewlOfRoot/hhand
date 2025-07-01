use anyhow::Ok;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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
    let config = toml::from_str(content.as_str())?;
    Ok(config)
}

pub fn save(config: Config) -> anyhow::Result<()> {
    let config_path = Path::new(CONFIG_PATH);
    save_to_path(config, config_path)?;
    Ok(())
}

fn save_to_path(config: Config, path: &Path) -> anyhow::Result<()> {
    let str_content = toml::to_string(&config)?;
    fs::write(path, str_content)?;
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
    fn test_load_returns_default_when_file_missing() {
        cleanup_test_config();
        let path = Path::new(TEST_CONFIG_PATH);
        // Temporarily change CONFIG_PATH for this test
        let config = load_or_default(path);
        assert!(config.is_ok());
        let config = config.unwrap();

        let default_config: Config = Config::default();
        assert_eq!(config.browser, default_config.browser);
    }

    #[test]
    fn test_load_reads_existing_config() {
        cleanup_test_config();
        let config_content = r#"
            browser = "Chrome"
        "#;
        fs::write(TEST_CONFIG_PATH, config_content).unwrap();

        // Temporarily change CONFIG_PATH for this test
        let config = load_from_path(Path::new(TEST_CONFIG_PATH));
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(matches!(config.browser, Browser::Chrome));

        cleanup_test_config();
    }

    #[test]
    fn test_save_writes_config_file() {
        cleanup_test_config();
        let config = Config {
            browser: Browser::Chrome,
        };
        let path = Path::new(TEST_CONFIG_PATH);

        // Save config to test file
        save_to_path(config, path).unwrap();

        // Read back and check
        let loaded = load_from_path(path).unwrap();
        assert!(matches!(loaded.browser, Browser::Chrome));

        cleanup_test_config();
    }

    #[test]
    fn test_save_and_load_cycle() {
        cleanup_test_config();
        let config = Config {
            browser: Browser::Firefox,
        };
        let path = Path::new(TEST_CONFIG_PATH);

        // Save using the save function
        save_to_path(config, path).unwrap();

        // Load and check
        let loaded = load_from_path(path).unwrap();
        assert!(matches!(loaded.browser, Browser::Firefox));

        cleanup_test_config();
    }
}
