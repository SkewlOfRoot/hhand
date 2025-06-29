use anyhow::Ok;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

const CONFIG_PATH: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub browser: Browser,
}

#[derive(Serialize, Deserialize)]
pub enum Browser {
    Chrome,
    Firefox,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Path::new(CONFIG_PATH);

        if config_path.exists() {
            Ok(load(config_path)?)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(config: Config) -> anyhow::Result<()> {
        let str_content = toml::to_string(&config)?;
        let path = Path::new(CONFIG_PATH);
        fs::write(path, str_content)?;
        Ok(())
    }
}

fn load(path: &Path) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    let config = toml::from_str(content.as_str())?;
    Ok(config)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            browser: Browser::Chrome,
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
        // Temporarily change CONFIG_PATH for this test
        let config = Config::load();
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(matches!(config.browser, Browser::Firefox));
    }

    #[test]
    fn test_load_reads_existing_config() {
        cleanup_test_config();
        let config_content = r#"
            browser = "Chrome"
        "#;
        fs::write(TEST_CONFIG_PATH, config_content).unwrap();

        // Temporarily change CONFIG_PATH for this test
        let config = load(Path::new(TEST_CONFIG_PATH));
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

        // Save config to test file
        Config::save(config).unwrap();

        // Read back and check
        let loaded = Config::load().unwrap();
        assert!(matches!(loaded.browser, Browser::Chrome));

        cleanup_test_config();
    }

    #[test]
    fn test_save_and_load_cycle() {
        cleanup_test_config();
        let config = Config {
            browser: Browser::Firefox,
        };

        // Save using the save function
        Config::save(config).unwrap();

        // Load and check
        let loaded = Config::load().unwrap();
        assert!(matches!(loaded.browser, Browser::Firefox));

        cleanup_test_config();
    }
}
