use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

use super::resource_file_path;

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

    if let Err(why) = file.write_all(json.as_bytes()) {
        panic!("failed to write bookmarks to file: {why}");
    }

    Ok(())
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
