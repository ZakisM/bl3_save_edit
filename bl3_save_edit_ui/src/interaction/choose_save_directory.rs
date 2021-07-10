use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use bl3_save_edit_core::file_helper::Bl3FileType;
use native_dialog::FileDialog;

pub async fn choose() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().context("failed to read home dir")?;

    #[cfg(target_os = "windows")]
    let default_dir = home_dir.join("Documents/My Games/Borderlands 3/Saved/SaveGames/");

    #[cfg(target_os = "macos")]
    let default_dir = "";

    #[cfg(target_os = "linux")]
    let default_dir = "/home/zak/IdeaProjects/bl3_save_edit/bl3_save_edit_core/test_files/";

    let mut file_dialog = FileDialog::new();

    if default_dir.exists() {
        file_dialog = file_dialog.set_location(&default_dir);
    }

    let res = file_dialog
        .show_open_single_dir()?
        .context("no directory was selected")?;

    Ok(res)
}

pub async fn load_files_in_directory(dir: Arc<PathBuf>) -> Result<()> {
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
                    .map(|p| p == "sav")
                    .is_some()
            {
                match tokio::fs::read(path).await {
                    Ok(data) => all_data.push(data),
                    Err(e) => eprintln!("{}", e),
                }
            }
        } else {
            break;
        }
    }

    let all_files: Vec<Bl3FileType> = tokio_rayon::spawn(move || {
        all_data
            .par_iter()
            .filter_map(|l| Bl3FileType::from_unknown_data(l).ok())
            .collect::<Vec<_>>()
    })
    .await;

    if let Some(end_time) = tokio::time::Instant::now().checked_duration_since(start_time) {
        println!(
            "Read {} files in {} milliseconds",
            all_files.len(),
            end_time.as_millis()
        );
    }

    for file in all_files {
        match file {
            Bl3FileType::PcSave(f) | Bl3FileType::Ps4Save(f) => println!(
                "Save: {} ({}) - Level {}",
                f.character_data.character.preferred_character_name,
                f.character_data.player_class,
                f.character_data.player_level
            ),
            Bl3FileType::PcProfile(f) | Bl3FileType::Ps4Profile(f) => {
                println!("Profile: Golden Keys {}", f.profile_data.golden_keys);
            }
        }
    }

    Ok(())
}
