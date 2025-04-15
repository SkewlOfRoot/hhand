use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

pub enum Control {
    ShouldExit,
    InputImportPath(String),
    PasteImportPath,
    Delete,
    InitiateImport,
    SetSearchState,
    SetImportState,
    SelectNextBookmark,
    SelectPreviousBookmark,
    InputSearch(String),
    OpenBookmark,
    ClearSearch,
    None,
}

enum Mode {
    Search,
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

        match self.mode {
            Mode::Import => match key.code {
                KeyCode::Esc => Control::ShouldExit,
                KeyCode::Char(value) => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        Control::PasteImportPath
                    } else {
                        Control::InputImportPath(value.to_string())
                    }
                }
                KeyCode::Backspace => Control::Delete,
                KeyCode::Enter => Control::InitiateImport,
                KeyCode::Insert => Control::SetSearchState,
                _ => Control::None,
            },
            Mode::Search => match key.code {
                KeyCode::Esc => Control::ShouldExit,
                KeyCode::Down => Control::SelectNextBookmark,
                KeyCode::Up => Control::SelectPreviousBookmark,
                KeyCode::Char(value) => Control::InputSearch(value.to_string()),
                KeyCode::Backspace => Control::Delete,
                KeyCode::Enter => Control::OpenBookmark,
                KeyCode::Insert => Control::SetImportState,
                KeyCode::Delete => Control::ClearSearch,
                _ => Control::None,
            },
        }
    }

    pub fn set_mode_search(&mut self) {
        self.mode = Mode::Search;
    }

    pub fn set_mode_import(&mut self) {
        self.mode = Mode::Import;
    }
}
