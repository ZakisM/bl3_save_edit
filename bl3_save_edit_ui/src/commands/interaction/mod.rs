use std::path::PathBuf;

use anyhow::{Context, Result};

pub mod choose_save_directory;
pub mod file_save;
pub mod manage_save;
pub mod settings;

#[cfg(not(target_os = "macos"))]
pub async fn choose_dir(existing_dir: PathBuf) -> Result<PathBuf> {
    use native_dialog::FileDialog;

    let mut file_dialog = FileDialog::new();

    if existing_dir.exists() {
        file_dialog = file_dialog.set_location(&existing_dir);
    }

    let res = file_dialog
        .show_open_single_dir()?
        .context("No folder was selected.")?;

    Ok(res)
}

#[cfg(target_os = "macos")]
pub async fn choose_dir(existing_dir: PathBuf) -> Result<PathBuf> {
    use native_dialog::{Dialog, OpenSingleDir};

    let mut default_dir = None;

    if existing_dir.exists() {
        let existing_dir_str = existing_dir.to_str().unwrap_or("");

        if !existing_dir_str.is_empty() {
            default_dir = Some(existing_dir_str);
        }
    }

    let dialog = OpenSingleDir { dir: default_dir };

    let res = dialog.show()?.context("No folder was selected.")?;

    Ok(res)
}
