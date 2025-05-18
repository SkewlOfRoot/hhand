use serde::{Deserialize, Serialize};
use std::{
    env::{self},
    fs,
    path::PathBuf,
    str::FromStr,
};

pub fn import_from_chrome() -> anyhow::Result<Vec<Bookmark>> {
    let local_app_data_path = if cfg!(target_os = "windows") {
        let path = match env::var("LOCALAPPDATA") {
            Ok(path) => path,
            Err(why) => panic!("{}", format!("Couldn't read LOCALAPPDATA: {}", why)),
        };

        let mut path = match PathBuf::from_str(&path) {
            Ok(p) => p,
            Err(why) => panic!(
                "{}",
                format!("Couldn't convert LOCALAPPDATA to path: {}", why)
            ),
        };

        path.push("Google/Chrome/User Data/Default/Bookmarks");
        path
    } else if cfg!(target_os = "linux") {
        let mut path = dirs::home_dir().expect("Could not determine home directory.");

        path.push(".config/google-chrome/Default/Bookmarks");
        path
    } else {
        panic!("Unsupported OS");
    };

    if !local_app_data_path.exists() {
        panic!(
            "{}",
            format!("The path {:#?} does not exist.", local_app_data_path)
        );
    }

    let content = fs::read_to_string(local_app_data_path).expect("Failed to read bookmarks file");

    let chrome_bookmarks: ChromeRoot =
        serde_json::from_str(&content).expect("Failed to parse JSON");

    let bookmarks: Vec<Bookmark> = unpack_chrome_roots(&chrome_bookmarks);
    Ok(bookmarks)
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
                    for bookmark in unpack_chrome_bookmarks(child) {
                        bookmarks.push(bookmark);
                    }
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
