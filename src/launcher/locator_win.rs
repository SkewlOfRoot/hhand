use std::{fs::read_dir, path::Path};

use super::LaunchableApp;

pub fn locate_apps() -> anyhow::Result<Vec<LaunchableApp>> {
    let mut apps: Vec<LaunchableApp> = Vec::new();

    if let Some(mut user_path) = dirs::home_dir() {
        user_path.push("AppData/Roaming/Microsoft/Windows/Start Menu/Programs/");

        if user_path.exists() {
            apps.extend(get_apps(&user_path)?);
        }
    }

    let system_path = Path::new("c:/ProgramData/Microsoft/Windows/Start Menu/Programs/");
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
            if file_path.extension().and_then(|s| s.to_str()) != Some("lnk") {
                continue; // Skip non-link files
            }

            let name = file_path
                .file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or_default();

            let path = file_path.to_str().unwrap_or_default();
            apps.push(LaunchableApp::new(name, path));
        } else if file_path.is_dir() {
            apps.extend(get_apps(&file_path)?);
        }
    }

    Ok(apps)
}
