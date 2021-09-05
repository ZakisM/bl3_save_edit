use std::path::PathBuf;

use anyhow::Result;
use chrono::Local;
use tracing::info;

use bl3_save_edit_core::bl3_profile::Bl3Profile;
use bl3_save_edit_core::bl3_save::Bl3Save;

pub async fn save_file(
    config_dir: PathBuf,
    output_file: PathBuf,
    output: Vec<u8>,
    existing_save: Bl3Save,
    new_save: Bl3Save,
) -> Result<Bl3Save> {
    info!(
        "Making a backup of existing save: {}",
        existing_save.file_name
    );

    let current_time = Local::now().format("%d-%m-%Y_%H.%M.%S");

    let backup_name = format!(
        "{}_{}-{}.sav",
        existing_save.character_data.player_class(),
        existing_save
            .character_data
            .character
            .preferred_character_name,
        current_time
    );

    let (existing_save_output, _) = existing_save.as_bytes()?;

    tokio::fs::write(config_dir.join(backup_name), existing_save_output).await?;

    info!("Saving file: {}", new_save.file_name);

    tokio::fs::write(output_file, output).await?;

    Ok(new_save)
}

pub async fn save_profile(
    config_dir: PathBuf,
    output_file: PathBuf,
    output: Vec<u8>,
    existing_profile: Bl3Profile,
    new_profile: Bl3Profile,
) -> Result<Bl3Profile> {
    info!(
        "Making a backup of existing profile: {}",
        existing_profile.file_name
    );

    let current_time = Local::now().format("%d-%m-%Y_%H.%M.%S");

    let backup_name = format!(
        "{}-{}.sav",
        existing_profile.file_name.replace(".sav", ""),
        current_time
    );

    let (existing_profile_output, _) = existing_profile.as_bytes()?;

    tokio::fs::write(config_dir.join(backup_name), existing_profile_output).await?;

    info!("Saving profile: {}", new_profile.file_name);

    tokio::fs::write(output_file, output).await?;

    Ok(new_profile)
}
