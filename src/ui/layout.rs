use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, HighlightSpacing, List, ListItem, Paragraph, StatefulWidget, Widget},
};

use crate::app::{App, AppState, StatusMessage};

const COLOR_TITLE_FG: Color = Color::Rgb(139, 233, 253);
const COLOR_FG: Color = Color::Rgb(80, 250, 123);
const COLOR_BG: Color = Color::Rgb(40, 42, 54);
const COLOR_SELECTED_BG: Color = Color::Rgb(189, 147, 249);
const COLOR_BORDER: Color = Color::Rgb(68, 71, 90);
const COLOR_ACCENT1: Color = Color::Rgb(80, 250, 123); // Green
const COLOR_ACCENT2: Color = Color::Rgb(255, 121, 198); // Pink
const COLOR_SUCCESS: Color = Color::Rgb(80, 250, 123);
const COLOR_ERROR: Color = Color::Rgb(255, 85, 85);

const SELECTED_STYLE: Style = Style::new()
    .fg(COLOR_FG)
    .bg(COLOR_SELECTED_BG)
    .add_modifier(Modifier::BOLD);

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
            AppState::App => {
                self.render_app_header(buf, header_area);
            }
        }
        self.render_footer(buf, footer_area);
    }
}

/// Functions for rendering UI
impl App {
    fn render_search_header(&self, buf: &mut Buffer, area: Rect) {
        let block = Block::bordered().title(self.title.as_str());
        Paragraph::new(self.input_str.clone())
            .block(block)
            .style(Style::default().bg(COLOR_BG))
            .render(area, buf);
    }

    fn render_import_header(&self, buf: &mut Buffer, area: Rect) {
        let block = Block::bordered().title(self.title.as_str());
        Paragraph::new(self.input_str.clone())
            .block(block)
            .style(Style::default().bg(COLOR_BG))
            .render(area, buf);
    }

    fn render_app_header(&self, buf: &mut Buffer, area: Rect) {
        let block = Block::bordered().title(self.title.as_str());
        Paragraph::new(self.input_str.clone())
            .block(block)
            .style(Style::default().bg(COLOR_BG))
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

        let block = Block::bordered()
            .title(Line::raw("Bookmarks ").left_aligned())
            .border_style(Style::default().fg(COLOR_TITLE_FG).bg(COLOR_BG))
            .bg(COLOR_BG);

        let list = List::new(list_items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.bookmark_list.state);
    }

    fn render_footer(&self, buf: &mut Buffer, area: Rect) {
        let [left_area, right_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(area);

        self.render_left_footer(buf, left_area);
        self.render_right_footer(buf, right_area);
    }

    fn render_left_footer(&self, buf: &mut Buffer, area: Rect) {
        let mode_spans = match self.state {
            AppState::Search => {
                vec![
                            Span::styled("Search mode", Style::default().fg(COLOR_ACCENT1)),
                            Span::styled(" | ", Style::default().fg(Color::White)),
                            Span::styled(
                                "(ESC) exit / (INS) switch mode / ↑↓ select bookmark / (ENTER) open bookmark",
                                Style::default().fg(COLOR_ACCENT2),
                            ),
                        ]
            }
            AppState::Import => vec![
                Span::styled("Import mode", Style::default().fg(COLOR_ACCENT1)),
                Span::styled(" | ", Style::default().fg(Color::White)),
                Span::styled(
                    "(ESC) exit / (INS) switch mode / (ENTER) init import",
                    Style::default().fg(COLOR_ACCENT2),
                ),
            ],
            AppState::App => vec![
                Span::styled("App mode", Style::default().fg(COLOR_ACCENT1)),
                Span::styled(" | ", Style::default().fg(Color::White)),
                Span::styled(
                    "(ESC) exit / (PgUp)/(PgDwn) switch mode / ↑↓ select item / (ENTER) execute",
                    Style::default().fg(COLOR_ACCENT2),
                ),
            ],
        };

        let block = Block::bordered().fg(COLOR_BORDER).bg(COLOR_BG);
        Paragraph::new(Line::from(mode_spans))
            .style(Style::default().bg(COLOR_BG).bold())
            .block(block)
            .render(area, buf);
    }

    fn render_right_footer(&self, buf: &mut Buffer, area: Rect) {
        let status_spans = vec![
            Span::styled("Status: ", Style::default()),
            match &self.status_message {
                StatusMessage::Success(msg) => {
                    Span::styled(msg, Style::default().fg(COLOR_SUCCESS))
                }
                StatusMessage::Error(err) => Span::styled(err, Style::default().fg(COLOR_ERROR)),
                StatusMessage::None => Span::styled("OK", Style::default().fg(COLOR_SUCCESS)),
            },
        ];

        let block = Block::bordered().fg(COLOR_BORDER).bg(COLOR_BG);
        Paragraph::new(Line::from(status_spans))
            .style(Style::default().bg(COLOR_BG).bold())
            .block(block)
            .render(area, buf);
    }
}
