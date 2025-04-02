use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{
        palette::{material::BLUE, tailwind::SLATE},
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

use crate::bookmarks::Bookmark;

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct App {
    should_exit: bool,
    pub bookmark_list: BookmarkList,
    pub search_str: String,
}

impl App {
    pub fn new(bookmarks: Vec<Bookmark>) -> App {
        App {
            should_exit: false,
            bookmark_list: BookmarkList {
                bookmarks,
                state: ListState::default(),
            },
            search_str: String::new(),
        }
    }

    pub fn select_next(&mut self) {
        self.bookmark_list.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.bookmark_list.state.select_previous();
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

    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Backspace => {
                self.search_str.pop();
            }
            KeyCode::Esc => self.should_exit = true,
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Char(value) => {
                self.search_str.push(value);
            }
            _ => {}
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, _] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .areas(area);

        self.render_header(buf, header_area);
        self.render_list(buf, main_area);
    }
}

// Rendering
impl App {
    fn render_header(&self, buf: &mut Buffer, header_area: Rect) {
        let search_block = Block::default().title("Search").borders(Borders::ALL);
        Paragraph::new(self.search_str.clone())
            .block(search_block)
            .render(header_area, buf);
    }

    fn render_list(&mut self, buf: &mut Buffer, main_area: Rect) {
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

        StatefulWidget::render(list, main_area, buf, &mut self.bookmark_list.state);
    }
}

pub struct BookmarkList {
    bookmarks: Vec<Bookmark>,
    state: ListState,
}
