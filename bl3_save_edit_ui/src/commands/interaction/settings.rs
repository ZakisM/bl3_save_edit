use std::path::PathBuf;

use anyhow::{bail, Result};

pub async fn open_dir(dir: PathBuf) -> Result<()> {
    if dir.exists() {
        open::that(&dir).map_err(anyhow::Error::new)
    } else {
        bail!("Folder does not exist.")
    }
}
