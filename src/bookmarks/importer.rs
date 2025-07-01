//! Bookmarks Importer
//!
//! This module provides a unified interface for importing bookmarks from different browsers.
//! It currently supports Google Chrome and Mozilla Firefox, delegating the actual import logic
//! to browser-specific submodules.
//!
//! # Supported Browsers
//! - Google Chrome
//! - Mozilla Firefox
//!
//! # Usage
//! Use [`import_from`] with a [`Browser`] variant to import bookmarks from the selected browser.
//!
//! # Errors
//! Returns an error if bookmarks cannot be imported from the selected browser, for example if
//! the bookmarks file or database cannot be found, or if there are issues reading or parsing data.

use anyhow::Result;
use serde::{Deserialize, Serialize};

mod chrome;
mod firefox;

/// Enum representing supported browsers for bookmark import.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Browser {
    Chrome,
    Firefox,
}

/// Represents a single bookmark with a name and URL.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bookmark {
    pub name: String,
    pub url: String,
}

impl Bookmark {
    /// Creates a new [`Bookmark`] with the given name and URL.
    pub fn new(name: &str, url: &str) -> Bookmark {
        Bookmark {
            name: name.to_string(),
            url: url.to_string(),
        }
    }
}

/// Imports bookmarks from the specified browser.
///
/// # Arguments
/// * `browser` - The browser to import bookmarks from (e.g., [`Browser::Chrome`] or [`Browser::Firefox`]).
///
/// # Returns
/// * `Ok(Vec<Bookmark>)` containing all imported bookmarks on success.
/// * `Err(anyhow::Error)` if bookmarks cannot be imported.
pub fn import_from(browser: &Browser) -> Result<Vec<Bookmark>> {
    match browser {
        Browser::Chrome => chrome::import(),
        Browser::Firefox => firefox::import(),
    }
}
