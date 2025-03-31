use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

const RESOURCE_FILE_NAME: &str = "hhand.resources.json";

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
        Err(why) => panic!("failed to parse 'a' tags in HTML document: {}", why),
    };

    let mut bookmarks: Vec<Bookmark> = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            let label = element.text().collect::<String>().trim().to_string();
            bookmarks.push(Bookmark {
                name: label,
                url: href.to_string(),
            });
        }
    }

    bookmarks
}

fn save_bookmarks(bookmarks: Vec<Bookmark>) -> anyhow::Result<()> {
    let json = serde_json::to_string(&bookmarks)?;

    let mut path = std::env::current_exe().expect("current EXE not found.");
    path.pop();
    path.push(RESOURCE_FILE_NAME);

    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(why) => panic!("failed to create resource file: {}", why),
    };

    match file.write_all(json.as_bytes()) {
        Ok(_) => println!("successfully imported {} bookmarks.", bookmarks.len()),
        Err(why) => panic!("failed to write bookmarks to file: {}", why),
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
    pub name: String,
    pub url: String,
}
