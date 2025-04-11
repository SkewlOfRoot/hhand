mod importer;
mod loader;

pub use importer::{import_from_file, Bookmark};
pub use loader::load_bookmarks;

use std::path::PathBuf;

const RESOURCE_FILE_NAME: &str = "hhand.resources.json";

pub fn resource_file_path() -> PathBuf {
    let mut path = match std::env::current_exe() {
        Ok(p) => p,
        Err(why) => panic!("faild to get current EXE path: {why}"),
    };
    path.pop();
    path.push(RESOURCE_FILE_NAME);
    path
}
