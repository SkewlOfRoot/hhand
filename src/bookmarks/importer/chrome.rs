//! Chrome Bookmarks Importer
//!
//! This module provides functionality to import bookmarks from Google Chrome profiles.
//! It locates the Chrome bookmarks file, parses its JSON structure, and extracts bookmark data
//! into a flat list of `Bookmark` structs.
//!
//! # Supported Platforms
//! - Linux: Locates the default Chrome bookmarks under `~/.config/google-chrome/Default/Bookmarks`.
//! - Windows: Locates the default Chrome bookmarks under `AppData/Local/Google/Chrome/User Data/Default/Bookmarks`,
//!   or falls back to `Profile 1` if the default does not exist.
//!
//! # Errors
//! Returns an error if the home directory cannot be determined, the bookmarks file cannot be found,
//! or if there are issues reading or parsing the file.

use std::{fs, path::PathBuf};

use super::Bookmark;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Imports bookmarks from the default Chrome profile.
///
/// # Returns
/// - `Ok(Vec<Bookmark>)` on success.
/// - `Err(anyhow::Error)` if bookmarks cannot be imported.
pub(crate) fn import() -> Result<Vec<Bookmark>> {
    let file_path = get_bookmarks_file_path()?;

    if !file_path.exists() {
        return Err(anyhow::anyhow!("The path {:?} does not exist.", file_path));
    }

    let content = fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read {:?}", file_path))?;
    let chrome_bookmarks: ChromeRoot = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse Chrome bookmarks at {:?}", file_path))?;

    let bookmarks: Vec<Bookmark> = unpack_chrome_roots(&chrome_bookmarks);
    Ok(bookmarks)
}

/// Determines the path to the Chrome bookmarks file for the default profile.
///
/// On Windows, checks both the "Default" and "Profile 1" directories.
/// On Linux, uses the default Chrome config directory.
///
/// # Returns
/// - `Ok(PathBuf)` with the path to the bookmarks file.
/// - `Err(anyhow::Error)` if the path cannot be determined.
fn get_bookmarks_file_path() -> Result<PathBuf> {
    if cfg!(target_os = "windows") {
        let mut default_path = get_home_dir()?;
        default_path.push("AppData/Local/Google/Chrome/User Data/Default/Bookmarks");

        let p = if default_path.exists() {
            default_path
        } else {
            let mut profile_path = get_home_dir()?;
            profile_path.push("AppData/Local/Google/Chrome/User Data/Profile 1/Bookmarks");
            profile_path
        };
        Ok(p)
    } else if cfg!(target_os = "linux") {
        let mut path = get_home_dir()?;
        path.push(".config/google-chrome/Default/Bookmarks");
        return Ok(path);
    } else {
        return Err(anyhow::anyhow!("Unsupported OS for importing bookmarks"));
    }
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

/// Unpacks all bookmarks from the Chrome bookmarks root structure.
///
/// # Arguments
/// - `root`: Reference to the deserialized Chrome bookmarks root.
///
/// # Returns
/// - `Vec<Bookmark>` containing all bookmarks found.
fn unpack_chrome_roots(root: &ChromeRoot) -> Vec<Bookmark> {
    let mut bookmarks: Vec<Bookmark> = Vec::new();
    bookmarks.extend(unpack_chrome_bookmarks(&root.roots.bookmark_bar));
    bookmarks.extend(unpack_chrome_bookmarks(&root.roots.other));
    bookmarks.extend(unpack_chrome_bookmarks(&root.roots.synced));
    bookmarks
}

/// Recursively unpacks bookmarks from a Chrome bookmark item.
///
/// # Arguments
/// - `bookmark_item`: Reference to a Chrome bookmark item node.
///
/// # Returns
/// - `Vec<Bookmark>` containing all bookmarks found under this node.
fn unpack_chrome_bookmarks(bookmark_item: &ChromeBookmarkItem) -> Vec<Bookmark> {
    let mut bookmarks: Vec<Bookmark> = Vec::new();

    match &bookmark_item.url {
        Some(url) => bookmarks.push(Bookmark::new(&bookmark_item.name, url)),
        None => {
            if let Some(children) = &bookmark_item.children {
                for child in children {
                    bookmarks.extend(unpack_chrome_bookmarks(child));
                }
            }
        }
    }

    bookmarks
}

#[derive(Serialize, Deserialize, Debug)]
struct ChromeRoot {
    roots: ChromeBookmarks,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChromeBookmarks {
    bookmark_bar: ChromeBookmarkItem,
    other: ChromeBookmarkItem,
    synced: ChromeBookmarkItem,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChromeBookmarkItem {
    name: String,
    url: Option<String>,
    children: Option<Vec<ChromeBookmarkItem>>,
}

#[test]
/// Tests that importing from Chrome returns at least one bookmark if the file exists.
/// This test will pass if bookmarks are present, or do nothing if the file is missing.
fn test_import_from_chrome() {
    match import() {
        Err(_) => {}
        Ok(bookmarks) => {
            assert!(!bookmarks.is_empty());
        }
    }
}
