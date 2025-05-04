use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::{self, stdout};

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
    cleanup_terminal()?;
    app_result
}

// Use this function to cleanup instead of ratatui::restore() as we need to call DisableMouseCapture when running application on Linux.
fn cleanup_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
