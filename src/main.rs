use std::{path::PathBuf, str::FromStr};

use anyhow::Ok;

mod bookmarks;

fn main() -> anyhow::Result<()> {
    bookmarks::import_from_file(PathBuf::from_str(
        "c:/temp/bookmarks/bookmarks_3_30_25.html",
    )?)?;

    Ok(())
}
