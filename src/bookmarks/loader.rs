use super::{importer::Bookmark, *};
use std::fs::{read_to_string, File};

pub fn load_bookmarks() -> anyhow::Result<Vec<Bookmark>, anyhow::Error> {
    let path = resource_file_path();

    if !path.exists() {
        File::create(resource_file_path()).expect("Could not create empty resource file.");
    }

    let json = match read_to_string(path) {
        Ok(content) => content,
        Err(why) => panic!("Failed to read content from file: {why}"),
    };

    if json.is_empty() {
        return Ok(Vec::new());
    }

    let bookmarks = match serde_json::from_str::<Vec<Bookmark>>(&json) {
        Ok(b) => b,
        Err(why) => panic!("Failed to deserialize to bookmarks: {why}"),
    };

    Ok(bookmarks)
}
