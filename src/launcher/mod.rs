mod launch;
mod locator;
mod locator_linux;
mod locator_win;

pub use locator::{locate_apps, LaunchableApp};
