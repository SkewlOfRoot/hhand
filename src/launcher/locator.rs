use std::{
    process::{Command, Stdio},
    str::FromStr,
};

use anyhow::Ok;

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
            name: String::from_str(name).unwrap(),
            exec_handle: String::from_str(exec_handle).unwrap(),
        }
    }

    pub fn launch(&self) -> anyhow::Result<()> {
        if cfg!(target_os = "windows") {
            self.launch_windows()?;
        } else if cfg!(target_os = "linux") {
            self.launch_linux()?
        } else {
            panic!("Unsupported OS");
        };
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
