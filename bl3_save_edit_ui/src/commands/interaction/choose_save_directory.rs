use std::ffi::OsStr;
use std::path::PathBuf;

use anyhow::{bail, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::{error, info};

use bl3_save_edit_core::file_helper::Bl3FileType;

use crate::commands::interaction::choose_dir;

#[cfg(not(target_os = "macos"))]
pub async fn choose(existing_dir: PathBuf) -> Result<PathBuf> {
    let home_dir = if existing_dir.exists() {
        existing_dir
    } else {
        dirs::home_dir().unwrap_or_default()
    };

    #[cfg(target_os = "windows")]
    let default_dir = home_dir.join("Documents/My Games/Borderlands 3/Saved/SaveGames/");

    #[cfg(target_os = "linux")]
    let default_dir = home_dir.join("IdeaProjects/bl3_save_edit/bl3_save_edit_core/test_files/");

    choose_dir(default_dir).await
}

#[cfg(target_os = "macos")]
pub async fn choose(existing_dir: PathBuf) -> Result<PathBuf> {
    let default_dir = if existing_dir.exists() {
        existing_dir
    } else {
        dirs::home_dir()
            .unwrap_or_default()
            .join("Library/Application Support/GearboxSoftware/OakGame/Saved/SaveGames")
    };

    choose_dir(default_dir).await
}

pub async fn load_files_in_directory(dir: PathBuf) -> Result<(PathBuf, Vec<Bl3FileType>)> {
    let start_time = tokio::time::Instant::now();

    let mut dirs = tokio::fs::read_dir(&*dir).await?;

    let mut all_data = vec![];

    while let Ok(entry) = dirs.next_entry().await {
        if let Some(entry) = entry {
            let path = entry.path();
            if !path.is_dir()
                && path
                    .extension()
                    .and_then(OsStr::to_str)
                    .and_then(|p| if p == "sav" { Some(()) } else { None })
                    .is_some()
            {
                match tokio::fs::read(&path).await {
                    Ok(data) => all_data.push((path, data)),
                    Err(e) => error!("{}", e),
                }
            }
        } else {
            break;
        }
    }

    let all_files: Vec<Bl3FileType> = tokio_rayon::spawn(move || {
        all_data
            .par_iter()
            .filter_map(|(file_name, data)| Bl3FileType::from_unknown_data(file_name, data).ok())
            .collect::<Vec<_>>()
    })
    .await;

    if all_files.is_empty() {
        bail!("No Save files or Profiles were found.")
    }

    if let Some(end_time) = tokio::time::Instant::now().checked_duration_since(start_time) {
        info!(
            "Read {} files in {} milliseconds",
            all_files.len(),
            end_time.as_millis()
        );
    }

    Ok((dir, all_files))
}
