use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

pub enum Control {
    ShouldExit,
    Input(String),
    PasteInput,
    Delete,
    InitiateImport,
    SetSearchState,
    SetImportState,
    SetAppState,
    SelectNextBookmark,
    SelectPreviousBookmark,
    OpenBookmark,
    Clear,
    None,
}

enum Mode {
    Search,
    App,
    Import,
}

pub struct InputHandler {
    mode: Mode,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler { mode: Mode::Search }
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
                Mode::Import => match key.code {
                    KeyCode::Enter => Control::InitiateImport,
                    KeyCode::Insert => Control::SetSearchState,
                    _ => Control::None,
                },
                Mode::Search => match key.code {
                    KeyCode::Down => Control::SelectNextBookmark,
                    KeyCode::Up => Control::SelectPreviousBookmark,
                    KeyCode::Char(value) => Control::Input(value.to_string()),
                    KeyCode::Enter => Control::OpenBookmark,
                    KeyCode::Insert => Control::SetImportState,

                    KeyCode::PageDown => Control::SetAppState,
                    KeyCode::PageUp => Control::SetAppState,
                    _ => Control::None,
                },
                Mode::App => match key.code {
                    KeyCode::PageDown => Control::SetSearchState,
                    KeyCode::PageUp => Control::SetSearchState,
                    KeyCode::Char(value) => Control::Input(value.to_string()),
                    _ => Control::None,
                },
            },
        }
    }

    pub fn set_mode_search(&mut self) {
        self.mode = Mode::Search;
    }

    pub fn set_mode_import(&mut self) {
        self.mode = Mode::Import;
    }

    pub fn set_mode_app(&mut self) {
        self.mode = Mode::App;
    }
}
