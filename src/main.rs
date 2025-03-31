use std::{path::PathBuf, str::FromStr};

use anyhow::Ok;

mod import;

fn main() -> anyhow::Result<()> {
    import::import_from_file(PathBuf::from_str(
        "c:/temp/bookmarks/bookmarks_3_30_25.html",
    )?)?;

    Ok(())
}
