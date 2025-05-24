use ini::Ini;
use std::{fs::read_dir, path::Path};

use super::LaunchableApp;

pub fn locate_apps() -> anyhow::Result<Vec<LaunchableApp>> {
    let mut apps: Vec<LaunchableApp> = Vec::new();

    if let Some(mut local_path) = dirs::home_dir() {
        local_path.push(".local/share/applications/");

        if local_path.exists() {
            apps.extend(get_apps(&local_path)?);
        }
    }

    let system_path = Path::new("/usr/share/applications/");
    if system_path.exists() {
        apps.extend(get_apps(system_path)?);
    }

    Ok(apps)
}

fn get_apps(path: &Path) -> anyhow::Result<Vec<LaunchableApp>> {
    let mut apps: Vec<LaunchableApp> = Vec::new();

    let entries = read_dir(path)?;
    for entry in entries {
        let file_path = match entry {
            Ok(e) => e.path(),
            Err(_) => continue, // Skip entries that cannot be read
        };

        if file_path.is_file() {
            if file_path.extension().and_then(|s| s.to_str()) != Some("desktop") {
                continue; // Skip non-desktop files
            }

            if let Some(e) = parse_ini_file(&file_path)? {
                apps.push(e);
            }
        } else if file_path.is_dir() {
            apps.extend(get_apps(&file_path)?);
        }
    }

    Ok(apps)
}

fn parse_ini_file(file_path: &Path) -> anyhow::Result<Option<LaunchableApp>> {
    let ini_file = Ini::load_from_file(file_path)?;
    if let Some(sec) = ini_file.section(Some("Desktop Entry")) {
        if let (Some(name), Some(exec)) = (sec.get("Name"), sec.get("Exec")) {
            return Ok(Some(LaunchableApp::new(name, exec)));
        }
    }
    Ok(None)
}
