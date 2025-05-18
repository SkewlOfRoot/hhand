use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::app::AppState;

pub enum Control {
    ShouldExit,
    Input(String),
    PasteInput,
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
    None,
}

pub struct InputHandler {
    mode: AppState,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            mode: AppState::Bookmarks,
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
            KeyCode::Char(value) => {
                if key.modifiers == KeyModifiers::CONTROL {
                    Control::PasteInput
                } else {
                    Control::Input(value.to_string())
                }
            }
            KeyCode::Backspace => Control::Delete,
            KeyCode::Delete => Control::Clear,
            _ => match self.mode {
                AppState::Bookmarks => match key.code {
                    KeyCode::PageDown => Control::SetLauncherState,
                    KeyCode::PageUp => Control::SetLauncherState,
                    KeyCode::Down => Control::SelectNextBookmark,
                    KeyCode::Up => Control::SelectPreviousBookmark,
                    KeyCode::Char(value) => Control::Input(value.to_string()),
                    KeyCode::Enter => Control::OpenBookmark,
                    _ => Control::None,
                },
                AppState::Launcher => match key.code {
                    KeyCode::PageDown => Control::SetBookmarksState,
                    KeyCode::PageUp => Control::SetBookmarksState,
                    KeyCode::Down => Control::SelectNextApp,
                    KeyCode::Up => Control::SelectPreviousApp,
                    KeyCode::Enter => Control::LaunchApp,
                    KeyCode::Char(value) => Control::Input(value.to_string()),
                    _ => Control::None,
                },
            },
        }
    }

    pub fn set_mode(&mut self, state: AppState) {
        self.mode = state;
    }
}
