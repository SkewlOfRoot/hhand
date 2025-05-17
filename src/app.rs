use arboard::Clipboard;
use ratatui::{widgets::ListState, DefaultTerminal};

use crate::{
    bookmarks::*,
    launcher::LaunchableApp,
    ui::{Control, InputHandler},
};

pub struct App {
    should_exit: bool,
    pub bookmark_list: BookmarkList,
    pub app_list: AppList,
    pub input_str: String,
    pub state: AppState,
    pub title: String,
    pub status_message: StatusMessage,
    input_handler: InputHandler,
}

pub enum AppState {
    Bookmarks,
    Projects,
    Launcher,
}

pub enum StatusMessage {
    Success(String),
    Error(String),
    None,
}

impl App {
    pub fn new(bookmarks: Vec<Bookmark>, apps: Vec<LaunchableApp>) -> App {
        let mut app = App {
            should_exit: false,
            bookmark_list: BookmarkList {
                bookmarks,
                state: ListState::default(),
            },
            app_list: AppList {
                apps,
                state: ListState::default(),
            },
            input_str: String::new(),
            state: AppState::Bookmarks,
            title: String::new(),
            status_message: StatusMessage::None,
            input_handler: InputHandler::new(),
        };
        app.set_state(AppState::Bookmarks);
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
                Control::SetBookmarksState => self.set_state(AppState::Bookmarks),
                Control::SetProjectsState => self.set_state(AppState::Projects),
                Control::SetLauncherState => self.set_state(AppState::Launcher),
                Control::SelectNextBookmark => self.select_next(),
                Control::SelectPreviousBookmark => self.select_previous(),
                Control::OpenBookmark => self.open_bookmark(),
                Control::Clear => self.clear_input(),

                Control::None => {}
            }
        }
        Ok(())
    }

    pub fn search_bookmarks(&self) -> Vec<Bookmark> {
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

    pub fn search_apps(&self) -> Vec<LaunchableApp> {
        self.app_list
            .apps
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
            let items = self.search_bookmarks();
            let item = &items[i];
            open::that(&item.url).unwrap();
        }
    }

    fn set_state(&mut self, new_state: AppState) {
        match new_state {
            AppState::Bookmarks => {
                self.title = "Search for bookmark".to_string();
                self.status_message = StatusMessage::Success(format!(
                    "Loaded {} bookmarks",
                    self.bookmark_list.bookmarks.len()
                ));
                self.input_handler.set_mode(AppState::Bookmarks);
            }

            AppState::Projects => {
                self.title = "Launch project".to_string();
                self.input_handler.set_mode(AppState::Projects);
                self.status_message = StatusMessage::None;
            }
            AppState::Launcher => {
                self.title = "Launch app".to_string();
                self.input_handler.set_mode(AppState::Launcher);
                self.status_message = StatusMessage::None;
            }
        }

        self.state = new_state;
        self.input_str.clear();
    }

    fn clear_input(&mut self) {
        self.input_str.clear();
    }
}

pub struct BookmarkList {
    bookmarks: Vec<Bookmark>,
    pub state: ListState,
}

pub struct AppList {
    apps: Vec<LaunchableApp>,
    pub state: ListState,
}
