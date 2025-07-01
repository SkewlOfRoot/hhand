use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

use std::{
    io::{self, stdout},
    process,
};

mod app;
mod bookmarks;
mod config;
mod launcher;
mod ui;

fn main() -> anyhow::Result<()> {
    let config = config::load()?;
    let bookmarks = match bookmarks::import_from(&config.browser) {
        Err(e) => {
            cleanup_terminal().ok();
            eprintln!(
                "Failed to import bookmarks from {:#?}: {}",
                &config.browser, e
            );
            process::exit(1);
        }
        Ok(b) => b,
    };

    let apps = match launcher::locate_apps() {
        Err(e) => {
            cleanup_terminal().ok();
            eprintln!("Failed to locate apps: {}", e);
            process::exit(1);
        }
        Ok(a) => a,
    };

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    let app_result = App::new(bookmarks, apps).run(terminal);
    cleanup_terminal()?;
    app_result
}

// Use this function to cleanup instead of ratatui::restore() as we need to call DisableMouseCapture when running application on Linux.
fn cleanup_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
