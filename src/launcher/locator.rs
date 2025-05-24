use anyhow::anyhow;
use std::process::{Command, Stdio};

use super::{locator_linux, locator_win};

#[derive(Debug, Clone)]
pub struct LaunchableApp {
    pub name: String,
    pub exec_handle: String,
}

pub fn locate_apps() -> anyhow::Result<Vec<LaunchableApp>> {
    let mut apps: Vec<LaunchableApp> = Vec::new();
    if cfg!(target_os = "windows") {
        apps.extend(locator_win::locate_apps()?);
    } else if cfg!(target_os = "linux") {
        apps.extend(locator_linux::locate_apps()?);
    } else {
        panic!("Unsupported OS");
    };

    Ok(apps)
}

impl LaunchableApp {
    pub fn new(name: &str, exec_handle: &str) -> Self {
        LaunchableApp {
            name: name.to_string(),
            exec_handle: exec_handle.to_string(),
        }
    }

    pub fn launch(&self) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            self.launch_windows()?;
        } else if cfg!(target_os = "linux") {
            self.launch_linux()?
        } else {
            return Err(anyhow!("Unsupported OS for launching applications"));
        }
        Ok(())
    }

    fn launch_windows(&self) -> anyhow::Result<()> {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", self.exec_handle.as_str()])
            .spawn()?;

        Ok(())
    }

    fn launch_linux(&self) -> anyhow::Result<()> {
        let cleaned = self
            .exec_handle
            .split_whitespace()
            .filter(|part| !part.starts_with('%'))
            .collect::<Vec<_>>();

        if !cleaned.is_empty() {
            Command::new(cleaned[0])
                .args(&cleaned[1..])
                .stdout(Stdio::null()) // Discard stdout
                .stderr(Stdio::null()) // Discard stderr
                .spawn()?;
        }
        Ok(())
    }
}

#[test]

fn test_locate_apps() -> anyhow::Result<()> {
    let result = locate_apps()?;
    assert!(!result.is_empty());
    Ok(())
}
