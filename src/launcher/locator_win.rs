use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

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

    let entries = read_dir(path).expect("Could not list directory.");
    for e in entries {
        let file_path: PathBuf = e.unwrap().path();
        if !file_path.is_dir() {
            let name = file_path.file_stem().unwrap_or_default();
            let path = file_path.to_str().unwrap_or_default();
            apps.push(LaunchableApp::new(name.to_str().unwrap_or_default(), path));
        } else if file_path.is_dir() {
            apps.extend(get_apps(&file_path)?);
        }
    }

    Ok(apps)
}
