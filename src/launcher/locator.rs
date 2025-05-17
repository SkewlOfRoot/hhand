use std::str::FromStr;

use super::locator_linux;

#[derive(Debug, Clone)]
pub struct LaunchableApp {
    pub name: String,
    pub exec_handle: String,
}

pub fn locate_apps() -> anyhow::Result<Vec<LaunchableApp>> {
    let mut apps: Vec<LaunchableApp> = Vec::new();

    if cfg!(target_os = "windows") {
        todo!();
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
}

#[test]

fn test_locate_apps() -> anyhow::Result<()> {
    let result = locate_apps()?;
    assert!(!result.is_empty());
    Ok(())
}
