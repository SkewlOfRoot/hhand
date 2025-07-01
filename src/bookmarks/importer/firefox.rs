//! Firefox Bookmarks Importer
//!
//! This module provides functionality to import bookmarks from Mozilla Firefox profiles.
//! It locates the Firefox profile directory, finds the `places.sqlite` database,
//! copies it to a temporary location (to avoid file locks), and extracts bookmark data
//! using an SQLite query.
//!
//! # Supported Platforms
//! - Linux: Locates the default Firefox profile under `~/.mozilla/firefox/`.
//! - Windows: Not yet implemented (see `todo!`).
//!
//! # Errors
//! Returns an error if the home directory cannot be determined, the profile or database
//! file cannot be found, or if there are issues reading or querying the database.

use crate::bookmarks::Bookmark;

use anyhow::Result;
use rusqlite::Connection;
use std::{fs, path::PathBuf};

/// Imports bookmarks from the default Firefox profile.
///
/// # Returns
/// - `Ok(Vec<Bookmark>)` on success.
/// - `Err(anyhow::Error)` if bookmarks cannot be imported.
pub(crate) fn import() -> Result<Vec<Bookmark>> {
    let file_path = get_places_file_path()?;
    let bookmarks = read_bookmarks(&file_path)?;
    Ok(bookmarks)
}

/// Determines the path to the `places.sqlite` file for the default Firefox profile.
///
/// On Linux, searches for a profile directory ending with `.default-release`.
/// On Windows, this is not yet implemented.
///
/// # Returns
/// - `Ok(PathBuf)` with the path to a temporary copy of `places.sqlite`.
/// - `Err(anyhow::Error)` if the profile or database cannot be found.
fn get_places_file_path() -> Result<PathBuf> {
    if cfg!(target_os = "windows") {
        let mut default_path = get_home_dir()?;
        default_path.push("AppData/Roaming/Mozilla/Firefox/Profiles/");

        todo!("Locate places.sqlite on Windows");
    } else if cfg!(target_os = "linux") {
        let mut base_path = get_home_dir()?;
        base_path.push(".mozilla/firefox/");

        Ok(places_file_path(base_path)?)
    } else {
        return Err(anyhow::anyhow!("Unsupported OS for importing bookmarks"));
    }
}

/// Locates the Firefox profile directory and copies `places.sqlite` to a temporary file.
///
/// # Arguments
/// - `local_base_path`: Path to the Firefox profiles directory.
///
/// # Returns
/// - `Ok(PathBuf)` with the path to the temporary copy of `places.sqlite`.
/// - `Err(anyhow::Error)` if the profile or database cannot be found.
fn places_file_path(local_base_path: PathBuf) -> Result<PathBuf> {
    // Locate Firefox profile directory.
    let profile_dir = match fs::read_dir(local_base_path)?
        .filter_map(Result::ok)
        .find(|e| e.path().is_dir() && e.path().ends_with("jdttdzef.default-release"))
    {
        None => return Err(anyhow::anyhow!("Firefox profile directory not found.")),
        Some(entry) => entry,
    };

    // Locate file 'places.sqlite' in profile directory.
    let places_entry = match fs::read_dir(profile_dir.path())?
        .filter_map(Result::ok)
        .find(|e| {
            e.path()
                .file_name()
                .is_some_and(|name| name == "places.sqlite")
        }) {
        None => return Err(anyhow::anyhow!("File 'places.sqlite' was not found.")),
        Some(entry) => entry,
    };

    let places_path = places_entry.path();

    // The file is locked while Firefox is running so we make a temp copy to read from.
    let temp_file_path = std::env::temp_dir().join("places_copy.sqlite");
    std::fs::copy(&places_path, &temp_file_path)?;

    Ok(temp_file_path)
}

/// Returns the current user's home directory.
///
/// # Returns
/// - `Ok(PathBuf)` with the home directory path.
/// - `Err(anyhow::Error)` if the home directory cannot be determined.
fn get_home_dir() -> Result<PathBuf> {
    let profile_path =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory."))?;
    Ok(profile_path)
}

/// Reads bookmarks from the given `places.sqlite` database file.
///
/// # Arguments
/// - `db_path`: Path to the SQLite database.
///
/// # Returns
/// - `Ok(Vec<Bookmark>)` with all found bookmarks.
/// - `Err(anyhow::Error)` if the database cannot be read or queried.
fn read_bookmarks(db_path: &PathBuf) -> Result<Vec<Bookmark>> {
    let conn = Connection::open(db_path)?;

    let mut statement = conn.prepare(
        "
        SELECT moz_bookmarks.title, moz_places.url
        FROM moz_bookmarks
        JOIN moz_places ON moz_bookmarks.fk = moz_places.id
        WHERE moz_bookmarks.type = 1
    ",
    )?;

    let rows = statement.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    let mut bookmarks: Vec<Bookmark> = Vec::new();
    for row in rows {
        let (title, url) = row?;
        bookmarks.push(Bookmark::new(title.as_str(), url.as_str()));
    }
    Ok(bookmarks)
}
