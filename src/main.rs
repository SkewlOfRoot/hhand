use app::App;
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io;

mod app;
mod bookmarks;
mod ui;

fn main() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    let bookmarks = match bookmarks::import_from_chrome() {
        Err(_) => {
            panic!("Failed to import Chrome bookmarks.")
        }
        Ok(b) => b,
    };

    let app_result = App::new(bookmarks).run(terminal);
    ratatui::restore();
    app_result
}
