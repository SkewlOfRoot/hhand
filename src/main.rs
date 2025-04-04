use std::io;

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use app::App;

mod app;
mod bookmarks;

fn main() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout(); // This is a special case. Normally using stdout is fine
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    let bookmarks = bookmarks::load_bookmarks()?;

    let app_result = App::new(bookmarks).run(terminal);
    ratatui::restore();
    app_result
}
