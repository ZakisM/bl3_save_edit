use std::path::PathBuf;

use anyhow::{bail, Result};
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;
use tracing::info;

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

    let current_dir = std::fs::read_dir(&*dir)?;

    let all_files = tokio_rayon::spawn(move || {
        current_dir
            .into_iter()
            .par_bridge()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                if let Ok(file_type) = entry.file_type() {
                    file_type.is_file() && entry.file_name().to_string_lossy().ends_with(".sav")
                } else {
                    false
                }
            })
            .filter_map(|entry| {
                let path = entry.path();

                std::fs::read(&path)
                    .map_err(anyhow::Error::from)
                    .and_then(|data| Bl3FileType::from_unknown_data(&path, &data))
                    .ok()
            })
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
