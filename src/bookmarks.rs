use anyhow::anyhow;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

const RESOURCE_FILE_NAME: &str = "hhand.resources.json";

pub fn load_bookmarks() -> anyhow::Result<Vec<Bookmark>, anyhow::Error> {
    let path = resource_file_path();

    if !path.exists() {
        return Err(anyhow!("Could not load bookmarks because the {} file does not exist. Please use the import command to import bookmarks to the resource file.", RESOURCE_FILE_NAME));
    }

    let json = match read_to_string(path) {
        Ok(content) => content,
        Err(why) => panic!("failed to read content from file: {why}"),
    };

    let bookmarks = match serde_json::from_str::<Vec<Bookmark>>(&json) {
        Ok(b) => b,
        Err(why) => panic!("failed to deserialize to bookmarks: {why}"),
    };

    Ok(bookmarks)
}

pub fn import_from_file(import_file: PathBuf) -> anyhow::Result<()> {
    let html = read_to_string(import_file).expect("failed to read content from import file.");
    let bookmarks = extract_bookmarks(html.as_str());

    save_bookmarks(bookmarks)?;
    Ok(())
}

/// Extracts bookmarks from an HTML file.
fn extract_bookmarks(html: &str) -> Vec<Bookmark> {
    let document = Html::parse_document(html);

    let selector = match Selector::parse("a") {
        Ok(sel) => sel,
        Err(why) => panic!("failed to parse 'a' tags in HTML document: {why}"),
    };

    let mut bookmarks: Vec<Bookmark> = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let name = element.text().collect::<String>().trim().to_string();
            bookmarks.push(Bookmark::new(name.as_str(), href));
        }
    }

    bookmarks
}

fn save_bookmarks(bookmarks: Vec<Bookmark>) -> anyhow::Result<()> {
    let json = serde_json::to_string(&bookmarks)?;

    let path = resource_file_path();

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(why) => panic!("failed to create resource file: {why}"),
    };

    match file.write_all(json.as_bytes()) {
        Ok(()) => println!("Successfully imported {} bookmarks.", bookmarks.len()),
        Err(why) => panic!("failed to write bookmarks to file: {why}"),
    }

    Ok(())
}

fn resource_file_path() -> PathBuf {
    let mut path = match std::env::current_exe() {
        Ok(p) => p,
        Err(why) => panic!("faild to get current EXE path: {why}"),
    };
    path.pop();
    path.push(RESOURCE_FILE_NAME);
    path
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
