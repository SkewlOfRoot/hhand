use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{
        palette::material::{GREEN, RED},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, Paragraph, StatefulWidget, Widget,
    },
};

use crate::app::{App, AppState, StatusMessage};

const COLOR_TITLE_FG: Color = Color::Rgb(235, 219, 178);
const COLOR_TITLE_BG: Color = Color::Rgb(26, 27, 38);
const COLOR_FG: Color = Color::Rgb(184, 187, 38);
const COLOR_BG: Color = Color::Rgb(40, 42, 54);
const COLOR_SELECTED_FG: Color = Color::Rgb(184, 187, 38);
const COLOR_SELECTED_BG: Color = Color::Rgb(250, 189, 47);

const TODO_HEADER_STYLE: Style = Style::new().fg(COLOR_TITLE_FG).bg(COLOR_TITLE_BG);
const SELECTED_STYLE: Style = Style::new()
    .fg(COLOR_SELECTED_FG)
    .bg(COLOR_SELECTED_BG)
    .add_modifier(Modifier::BOLD);

const SUCCESS_STYLE: Style = Style::new().fg(GREEN.c800).add_modifier(Modifier::BOLD);
const ERROR_STYLE: Style = Style::new().fg(RED.c800).add_modifier(Modifier::BOLD);

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
                Style::default().fg(COLOR_FG),
            ))));
        }

        let block = Block::new()
            .title(Line::raw("Bookmarks").left_aligned())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(COLOR_BG);

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
