use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

pub fn import_from_chrome() -> Result<Vec<Bookmark>> {
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

fn get_home_dir() -> Result<PathBuf> {
    let profile_path =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory."))?;
    Ok(profile_path)
}

fn unpack_chrome_roots(root: &ChromeRoot) -> Vec<Bookmark> {
    let mut bookmarks: Vec<Bookmark> = Vec::new();
    bookmarks.extend(unpack_chrome_bookmarks(&root.roots.bookmark_bar));
    bookmarks.extend(unpack_chrome_bookmarks(&root.roots.other));
    bookmarks.extend(unpack_chrome_bookmarks(&root.roots.synced));
    bookmarks
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookmark {
    pub name: String,
    pub url: String,
}

impl Bookmark {
    pub fn new(name: &str, url: &str) -> Bookmark {
        Bookmark {
            name: name.to_string(),
            url: url.to_string(),
        }
    }
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

fn test_import_from_chrome() {
    match import_from_chrome() {
        Err(_) => {}
        Ok(bookmarks) => {
            assert!(!bookmarks.is_empty());
        }
    }
}
