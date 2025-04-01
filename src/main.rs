use std::{io, path::PathBuf, str::FromStr};

use bookmarks::Bookmark;
use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};

use ui::ui;

mod bookmarks;
mod ui;

fn main() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    bookmarks::import_from_file(PathBuf::from_str(
        "c:/temp/bookmarks/bookmarks_3_30_25.html",
    )?)?;

    let bookmarks = bookmarks::load_bookmarks()?;

    let mut app = App::new(bookmarks);
    let res = run_app(&mut terminal, &mut app);

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match key.code {
                KeyCode::Backspace => {
                    app.search_str.pop();
                }
                KeyCode::Esc => {
                    return Ok(false);
                }
                KeyCode::Char(value) => {
                    app.search_str.push(value);
                }
                _ => {}
            }
        }
    }
}

pub struct App {
    bookmarks: Vec<Bookmark>,
    search_str: String,
}

impl App {
    pub fn new(bookmarks: Vec<Bookmark>) -> App {
        App {
            bookmarks,
            search_str: String::new(),
        }
    }

    pub fn search(&self) -> &Vec<Bookmark> {
        &self.bookmarks
    }
}
