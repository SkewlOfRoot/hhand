use std::{path::PathBuf, str::FromStr};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{
        palette::{
            material::{BLUE, GREEN, RED},
            tailwind::SLATE,
        },
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
    DefaultTerminal,
};

use crate::bookmarks::{self, Bookmark};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const SUCCESS_STYLE: Style = Style::new().fg(GREEN.c800).add_modifier(Modifier::BOLD);
const ERROR_STYLE: Style = Style::new().fg(RED.c800).add_modifier(Modifier::BOLD);

pub struct App {
    should_exit: bool,
    bookmark_list: BookmarkList,
    search_str: String,
    state: AppState,
    title: String,
    import_path: String,
    status_message: StatusMessage,
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

    fn search(&self) -> Vec<Bookmark> {
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

    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match self.state {
            AppState::Import => match key.code {
                KeyCode::Esc => self.should_exit = true,
                KeyCode::Char(value) => {
                    self.import_path.push(value);
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
        self.title = "Enter import file path".to_string();
        self.status_message = StatusMessage::None;
    }

    fn set_search_state(&mut self) {
        self.state = AppState::Search;
        self.search_str.clear();
        self.title = "Search".to_string();
        self.status_message = StatusMessage::None;
    }

    fn initiate_import(&mut self) {
        if let Ok(path) = PathBuf::from_str(&self.import_path) {
            if path.exists() {
                if let Err(why) = bookmarks::import_from_file(path) {
                    panic!("Failed to import bookmarks from file: {why}");
                } else {
                    self.import_path.clear();
                    self.status_message = StatusMessage::Success(format!(
                        "Successfully imported {} bookmarks.",
                        self.bookmark_list.bookmarks.len()
                    ));
                }
            } else {
                self.status_message = StatusMessage::Error(format!(
                    "Could not find import file at path '{}'.",
                    &self.import_path
                ));
            }
        } else {
            self.status_message =
                StatusMessage::Error("Unable to construct the import file path.".to_string());
        }
    }

    fn clear_search(&mut self) {
        self.search_str.clear();
    }
}

/// Implement the Widget trait for App
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .areas(area);

        match self.state {
            AppState::Import => self.render_import_header(buf, header_area),
            AppState::Search => {
                self.render_search_header(buf, header_area);
                self.render_search_list(buf, main_area)
            }
        }
        self.render_footer(buf, footer_area);
    }
}

/// Functions for rendering UI
impl App {
    fn render_search_header(&self, buf: &mut Buffer, area: Rect) {
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL);
        Paragraph::new(self.search_str.clone())
            .block(block)
            .render(area, buf);
    }

    fn render_import_header(&self, buf: &mut Buffer, area: Rect) {
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL);
        Paragraph::new(self.import_path.clone())
            .block(block)
            .render(area, buf);
    }

    fn render_search_list(&mut self, buf: &mut Buffer, area: Rect) {
        let mut list_items = Vec::<ListItem>::new();
        let matches = self.search();

        for m in matches {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{: <40} : {}", m.name, m.url),
                Style::default().fg(Color::Yellow),
            ))));
        }

        let block = Block::new()
            .title(Line::raw("Bookmarks").left_aligned())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        let list = List::new(list_items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.bookmark_list.state);
    }

    fn render_footer(&self, buf: &mut Buffer, area: Rect) {
        let message: &str;
        let style;

        match &self.status_message {
            StatusMessage::Success(msg) => {
                message = msg;
                style = SUCCESS_STYLE;
            }
            StatusMessage::Error(err) => {
                message = err;
                style = ERROR_STYLE;
            }
            StatusMessage::None => {
                message = "";
                style = SUCCESS_STYLE;
            }
        }

        Paragraph::new(message).style(style).render(area, buf);
    }
}

pub struct BookmarkList {
    bookmarks: Vec<Bookmark>,
    state: ListState,
}
