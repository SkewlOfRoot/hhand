use arboard::Clipboard;
use ratatui::{widgets::ListState, DefaultTerminal};

use crate::{
    bookmarks::*,
    ui::{Control, InputHandler},
};

pub struct App {
    should_exit: bool,
    pub bookmark_list: BookmarkList,
    pub input_str: String,
    pub state: AppState,
    pub title: String,
    pub status_message: StatusMessage,
    input_handler: InputHandler,
}

pub enum AppState {
    Search,
    App,
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
            input_str: String::new(),
            state: AppState::Search,
            title: String::new(),
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
                Control::Input(val) => self.input_str.push_str(val.as_str()),
                Control::PasteInput => self.paste_to_input(),
                Control::Delete => {
                    self.input_str.pop();
                }
                Control::SetSearchState => self.set_search_state(),
                Control::SelectNextBookmark => self.select_next(),
                Control::SelectPreviousBookmark => self.select_previous(),
                Control::OpenBookmark => self.open_bookmark(),
                Control::Clear => self.clear_input(),
                Control::SetAppState => self.set_app_state(),
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
                !&self.input_str.is_empty()
                    && b.name
                        .to_uppercase()
                        .contains(&self.input_str.to_uppercase())
            })
            .cloned()
            .collect()
    }

    fn paste_to_input(&mut self) {
        match Clipboard::new() {
            Err(why) => {
                self.status_message =
                    StatusMessage::Error(format!("Failed to initialize clipboard: {why}"))
            }
            Ok(mut clipboard) => {
                if let Ok(text) = clipboard.get_text() {
                    self.input_str.push_str(text.as_str());
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

    fn set_search_state(&mut self) {
        self.state = AppState::Search;
        self.input_handler.set_mode_search();
        self.input_str.clear();
        self.title = "Search for bookmark".to_string();
        self.status_message = StatusMessage::None;
    }

    fn set_app_state(&mut self) {
        self.state = AppState::App;
        self.input_handler.set_mode_app();
        self.input_str.clear();
        self.title = "Enter app command".to_string();
        self.status_message = StatusMessage::None;
    }

    fn clear_input(&mut self) {
        self.input_str.clear();
    }
}

pub struct BookmarkList {
    bookmarks: Vec<Bookmark>,
    pub state: ListState,
}
