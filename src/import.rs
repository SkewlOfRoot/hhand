use anyhow::Ok;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;

pub fn import_from_file(import_file: PathBuf) -> anyhow::Result<()> {
    let html = read_to_string(import_file).expect("failed to read content from import file.");
    let bookmarks = extract_bookmarks(html.as_str());

    let json = serde_json::to_string(&bookmarks)?;

    println!("{}", json);

    Ok(())
}

fn extract_bookmarks(html: &str) -> Vec<Bookmark> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Bookmark {
    pub name: String,
    pub url: String,
}
