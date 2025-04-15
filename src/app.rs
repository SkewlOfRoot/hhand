use arboard::Clipboard;
use ratatui::{widgets::ListState, DefaultTerminal};
use std::{path::PathBuf, str::FromStr};

use crate::{
    bookmarks::{self, *},
    ui::{Control, InputHandler},
};

pub struct App {
    should_exit: bool,
    pub bookmark_list: BookmarkList,
    pub search_str: String,
    pub state: AppState,
    pub title: String,
    pub import_path: String,
    pub status_message: StatusMessage,
    input_handler: InputHandler,
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
            input_handler: InputHandler::new(),
        };
        app.set_search_state();
        app
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;
            let control: Control = self.input_handler.read();

            match control {
                Control::ShouldExit => self.should_exit = true,
                Control::InputImportPath(val) => self.import_path.push_str(val.as_str()),
                Control::PasteImportPath => self.paste_import_path(),
                Control::Delete => match self.state {
                    AppState::Search => {
                        self.search_str.pop();
                    }
                    AppState::Import => {
                        self.import_path.pop();
                    }
                },
                Control::InitiateImport => self.initiate_import(),
                Control::SetSearchState => self.set_search_state(),
                Control::SetImportState => self.set_import_state(),
                Control::SelectNextBookmark => self.select_next(),
                Control::SelectPreviousBookmark => self.select_previous(),
                Control::InputSearch(val) => self.search_str.push_str(val.as_str()),
                Control::OpenBookmark => self.open_bookmark(),
                Control::ClearSearch => self.clear_search(),
                Control::None => {}
            }
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

    fn paste_import_path(&mut self) {
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

    fn select_next(&mut self) {
        self.bookmark_list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.bookmark_list.state.select_previous();
    }

    fn open_bookmark(&self) {
        if let Some(i) = self.bookmark_list.state.selected() {
            let items = self.search();
            let item = &items[i];
            open::that(&item.url).unwrap();
        }
    }

    fn set_import_state(&mut self) {
        self.state = AppState::Import;
        self.input_handler.set_mode_import();
        self.title = "Enter import file path".to_string();
        self.status_message = StatusMessage::None;
    }

    fn set_search_state(&mut self) {
        self.state = AppState::Search;
        self.input_handler.set_mode_search();
        self.search_str.clear();
        self.title = "Search for bookmark".to_string();
        self.status_message = StatusMessage::None;
    }

    fn initiate_import(&mut self) {
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

    fn import_bookmarks(&mut self, path: PathBuf) {
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

                self.load_bookmarks();
            }
        }
    }

    fn load_bookmarks(&mut self) {
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

    fn clear_search(&mut self) {
        self.search_str.clear();
    }
}

pub struct BookmarkList {
    bookmarks: Vec<Bookmark>,
    pub state: ListState,
}
