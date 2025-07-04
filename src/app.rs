use arboard::Clipboard;
use ratatui::{widgets::ListState, DefaultTerminal};

use crate::{
    bookmarks::*,
    config::{self, Config},
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
    pub config_manager: ConfigManager,
    input_handler: InputHandler,
}

pub enum AppState {
    Bookmarks,
    Launcher,
}

pub enum StatusMessage {
    Success(String),
    Error(String),
    None,
}

impl App {
    pub fn new(bookmarks: Vec<Bookmark>, apps: Vec<LaunchableApp>, config: Config) -> App {
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
            config_manager: ConfigManager::new(config),
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
                Control::_PasteInput => self.paste_to_input(),
                Control::Delete => {
                    self.input_str.pop();
                }
                Control::SetBookmarksState => self.set_state(AppState::Bookmarks),
                Control::SetLauncherState => self.set_state(AppState::Launcher),
                Control::SelectNextBookmark => self.bookmark_list.state.select_next(),
                Control::SelectPreviousBookmark => self.bookmark_list.state.select_previous(),
                Control::OpenBookmark => self.open_bookmark()?,
                Control::Clear => self.clear_input(),
                Control::ConfigVisible(visible) => self.set_config_visibile(visible),
                Control::None => {}
                Control::SelectNextApp => self.app_list.state.select_next(),
                Control::SelectPreviousApp => self.app_list.state.select_previous(),
                Control::LaunchApp => self.launch_app()?,
                Control::ConfigNext => self.config_manager.next(),
                Control::ConfigPrevious => self.config_manager.previous(),
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

    fn open_bookmark(&self) -> anyhow::Result<()> {
        if let Some(i) = self.bookmark_list.state.selected() {
            let items = self.search_bookmarks();
            if i < items.len() {
                let item = &items[i];
                open::that(&item.url)?;
            }
        }
        Ok(())
    }

    fn launch_app(&self) -> anyhow::Result<()> {
        if let Some(i) = self.app_list.state.selected() {
            let items = self.search_apps();
            if i < items.len() {
                let item = &items[i];
                item.launch()?;
            }
        }
        Ok(())
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
            AppState::Launcher => {
                self.title = "Launch app".to_string();
                self.input_handler.set_mode(AppState::Launcher);
                self.status_message =
                    StatusMessage::Success(format!("Located {} apps", self.app_list.apps.len()));
            }
        }

        self.state = new_state;
        self.input_str.clear();
    }

    fn clear_input(&mut self) {
        self.input_str.clear();
    }

    fn set_config_visibile(&mut self, visible: bool) {
        self.config_manager.is_visible = visible;
        self.input_handler.set_config_visible(visible);
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

pub struct ConfigManager {
    pub is_visible: bool,
    pub active_element: ConfigElement,
    pub config: Config,
}

pub enum ConfigElement {
    Browser,
    Ok,
    Cancel,
}

impl ConfigManager {
    fn new(config: Config) -> Self {
        ConfigManager {
            is_visible: false,
            active_element: ConfigElement::Browser,
            config,
        }
    }
}

impl ConfigManager {
    pub fn next(&mut self) {
        match self.active_element {
            ConfigElement::Browser => self.active_element = ConfigElement::Ok,
            ConfigElement::Ok => self.active_element = ConfigElement::Cancel,
            ConfigElement::Cancel => self.active_element = ConfigElement::Browser,
        }
    }
    pub fn previous(&mut self) {
        match self.active_element {
            ConfigElement::Browser => self.active_element = ConfigElement::Cancel,
            ConfigElement::Ok => self.active_element = ConfigElement::Browser,
            ConfigElement::Cancel => self.active_element = ConfigElement::Ok,
        }
    }

    pub fn save(self) -> anyhow::Result<()> {
        config::save(&self.config)?;
        Ok(())
    }
}
