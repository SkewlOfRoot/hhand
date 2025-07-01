use anyhow::Result;
use config::Browser;
use serde::{Deserialize, Serialize};

use crate::config;

mod chrome;
mod firefox;

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

pub fn import_from(browser: &Browser) -> Result<Vec<Bookmark>> {
    match browser {
        Browser::Chrome => chrome::import(),
        Browser::Firefox => firefox::import(),
    }
}
