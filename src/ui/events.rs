use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};

use crate::app::{App, AppState};

impl App {
    pub fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.state {
            AppState::Import => match key.code {
                KeyCode::Esc => self.should_exit = true,
                KeyCode::Char(value) => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        self.paste_import_path();
                    } else {
                        self.import_path.push(value);
                    }
                }
                KeyCode::Backspace => {
                    self.import_path.pop();
                }
                KeyCode::Enter => self.initiate_import(),
                KeyCode::Insert => self.set_search_state(),
                _ => {}
            },
            AppState::Search => match key.code {
                KeyCode::Esc => self.should_exit = true,
                KeyCode::Down => self.select_next(),
                KeyCode::Up => self.select_previous(),
                KeyCode::Char(value) => {
                    self.search_str.push(value);
                }
                KeyCode::Backspace => {
                    self.search_str.pop();
                }
                KeyCode::Enter => self.open_bookmark(),
                KeyCode::Insert => self.set_import_state(),
                KeyCode::Delete => self.clear_search(),
                _ => {}
            },
        }
    }
}
