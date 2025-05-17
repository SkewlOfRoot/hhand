use ini::Ini;
use std::{fs::read_dir, path::PathBuf};

use super::LaunchableApp;

pub fn locate_apps() -> anyhow::Result<Vec<LaunchableApp>> {
    let mut apps: Vec<LaunchableApp> = Vec::new();

    apps.extend(get_local_apps()?);
    apps.extend(get_user_apps());

    Ok(apps)
}

fn get_local_apps() -> anyhow::Result<Vec<LaunchableApp>> {
    let mut apps: Vec<LaunchableApp> = Vec::new();

    let mut dir_path = dirs::home_dir().expect("Could not determine home directory.");
    dir_path.push(".local/share/applications/");

    if dir_path.exists() {
        let entries = read_dir(dir_path).expect("Could not list directory.");
        println!("hello");
        for e in entries {
            let file_path = e.unwrap().path();

            if let Some(e) = parse_ini_file(&file_path)? {
                println!("found app: {:#?}", &e);
                apps.push(e);
            }
        }
    }

    Ok(apps)
}

fn parse_ini_file(file_path: &PathBuf) -> anyhow::Result<Option<LaunchableApp>> {
    let ini_file = Ini::load_from_file(file_path)?;
    match ini_file.section(Some("Desktop Entry")) {
        None => Ok(None),
        Some(sec) => {
            let name = sec.get("Name").unwrap();
            let exec = sec.get("Exec").unwrap();
            Ok(Some(LaunchableApp::new(name, exec)))
        }
    }
}

fn get_user_apps() -> Vec<LaunchableApp> {
    let apps: Vec<LaunchableApp> = Vec::new();

    apps
}
