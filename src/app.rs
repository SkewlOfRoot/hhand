use arboard::Clipboard;
use crossterm::event::{self, Event};
use ratatui::{widgets::ListState, DefaultTerminal};
use std::{path::PathBuf, str::FromStr};

use crate::bookmarks::{self, *};

pub struct App {
    pub should_exit: bool,
    pub bookmark_list: BookmarkList,
    pub search_str: String,
    pub state: AppState,
    pub title: String,
    pub import_path: String,
    pub status_message: StatusMessage,
}

pub enum AppState {
    Search,
    Import,
}

pub enum StatusMessage {
    Success(String),
    Error(String),
    None,
}

impl App {
    pub fn new(bookmarks: Vec<Bookmark>) -> App {
        let mut app = App {
            should_exit: false,
            bookmark_list: BookmarkList {
                bookmarks,
                state: ListState::default(),
            },
            search_str: String::new(),
            state: AppState::Search,
            title: String::new(),
            import_path: String::new(),
            status_message: StatusMessage::None,
        };
        app.set_search_state();
        app
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    pub fn search(&self) -> Vec<Bookmark> {
        self.bookmark_list
            .bookmarks
            .iter()
            .filter(|b| {
                !&self.search_str.is_empty()
                    && b.name
                        .to_uppercase()
                        .contains(&self.search_str.to_uppercase())
            })
            .cloned()
            .collect()
    }

    pub fn paste_import_path(&mut self) {
        match Clipboard::new() {
            Err(why) => {
                self.status_message =
                    StatusMessage::Error(format!("Failed to initialize clipboard: {why}"))
            }
            Ok(mut clipboard) => {
                if let Ok(text) = clipboard.get_text() {
                    self.import_path.push_str(text.as_str());
                }
            }
        }
    }

    pub fn select_next(&mut self) {
        self.bookmark_list.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.bookmark_list.state.select_previous();
    }

    pub fn open_bookmark(&self) {
        if let Some(i) = self.bookmark_list.state.selected() {
            let items = self.search();
            let item = &items[i];
            open::that(&item.url).unwrap();
        }
    }

    pub fn set_import_state(&mut self) {
        self.state = AppState::Import;
        self.title = "Enter import file path".to_string();
        self.status_message = StatusMessage::None;
    }

    pub fn set_search_state(&mut self) {
        self.state = AppState::Search;
        self.search_str.clear();
        self.title = "Search".to_string();
        self.status_message = StatusMessage::None;
    }

    pub fn initiate_import(&mut self) {
        match PathBuf::from_str(&self.import_path) {
            Ok(path) => {
                if path.exists() {
                    self.import_bookmarks(path);
                } else {
                    self.status_message = StatusMessage::Error(format!(
                        "Could not find import file at path '{}'",
                        &self.import_path
                    ));
                }
            }
            Err(_) => {
                self.status_message =
                    StatusMessage::Error("Unable to construct the import file path.".to_string());
            }
        }
    }

    pub fn import_bookmarks(&mut self, path: PathBuf) {
        match bookmarks::import_from_file(path) {
            Err(why) => {
                self.status_message =
                    StatusMessage::Error(format!("Failed to import bookmarks from file: {why}"));
            }
            Ok(res) => {
                self.import_path.clear();
                self.status_message = StatusMessage::Success(format!(
                    "Successfully imported {} bookmarks.",
                    res.no_of_imported_items
                ));

                self.reload_bookmarks();
            }
        }
    }

    pub fn reload_bookmarks(&mut self) {
        match bookmarks::load_bookmarks() {
            Err(why) => {
                self.status_message = StatusMessage::Error(format!(
                    "Failed to load bookmarks from resource file: {why}"
                ));
            }
            Ok(res) => {
                self.bookmark_list.bookmarks = res;
            }
        }
    }

    pub fn clear_search(&mut self) {
        self.search_str.clear();
    }
}

pub struct BookmarkList {
    bookmarks: Vec<Bookmark>,
    pub state: ListState,
}
