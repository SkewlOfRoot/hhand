use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::AppState;

pub enum Control {
    ShouldExit,
    Input(String),
    _PasteInput,
    Delete,
    SetBookmarksState,
    SetLauncherState,
    SelectNextBookmark,
    SelectPreviousBookmark,
    SelectNextApp,
    SelectPreviousApp,
    OpenBookmark,
    LaunchApp,
    Clear,
    ConfigVisible(bool),
    None,
    ConfigNext,
    ConfigPrevious,
}

pub struct InputHandler {
    mode: AppState,
    config_visible: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            mode: AppState::Bookmarks,
            config_visible: false,
        }
    }

    pub fn read(&self) -> Control {
        let key = match event::read().unwrap() {
            Event::Key(key) => key,
            _ => return Control::None,
        };

        if key.kind != KeyEventKind::Press {
            return Control::None;
        }

        match key.code {
            KeyCode::Esc => Control::ShouldExit,
            _ => {
                if key.code == KeyCode::F(1) {
                    return Control::ConfigVisible(!self.config_visible);
                }

                if self.config_visible {
                    return match key.code {
                        KeyCode::Down => Control::ConfigNext,
                        KeyCode::Right => Control::ConfigNext,
                        KeyCode::Up => Control::ConfigPrevious,
                        KeyCode::Left => Control::ConfigPrevious,
                        _ => Control::None,
                    };
                }

                match self.mode {
                    AppState::Bookmarks => match key.code {
                        KeyCode::PageDown => Control::SetLauncherState,
                        KeyCode::PageUp => Control::SetLauncherState,
                        KeyCode::Down => Control::SelectNextBookmark,
                        KeyCode::Up => Control::SelectPreviousBookmark,
                        KeyCode::Backspace => Control::Delete,
                        KeyCode::Delete => Control::Clear,
                        KeyCode::Enter => Control::OpenBookmark,
                        KeyCode::Char(value) => Control::Input(value.to_string()),
                        _ => Control::None,
                    },
                    AppState::Launcher => match key.code {
                        KeyCode::PageDown => Control::SetBookmarksState,
                        KeyCode::PageUp => Control::SetBookmarksState,
                        KeyCode::Down => Control::SelectNextApp,
                        KeyCode::Up => Control::SelectPreviousApp,
                        KeyCode::Backspace => Control::Delete,
                        KeyCode::Delete => Control::Clear,
                        KeyCode::Enter => Control::LaunchApp,
                        KeyCode::Char(value) => Control::Input(value.to_string()),
                        _ => Control::None,
                    },
                }
            }
        }
    }

    pub fn set_mode(&mut self, state: AppState) {
        self.mode = state;
    }

    pub fn set_config_visible(&mut self, visible: bool) {
        self.config_visible = visible;
    }
}
