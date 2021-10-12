use std::path::PathBuf;

use anyhow::Result;
use chrono::Local;
use tracing::info;

use bl3_save_edit_core::bl3_profile::Bl3Profile;
use bl3_save_edit_core::bl3_save::Bl3Save;
use bl3_save_edit_core::file_helper::Bl3FileType;

use crate::commands::interaction::choose_save_directory;
use crate::state_mappers;

pub async fn save_file(
    backup_dir: PathBuf,
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

    let backup_name = sanitize_filename::sanitize(backup_name);

    let (existing_save_output, _) = existing_save.as_bytes()?;

    tokio::fs::write(backup_dir.join(backup_name), existing_save_output).await?;

    info!("Saving file: {}", new_save.file_name);

    tokio::fs::write(output_file, output).await?;

    Ok(new_save)
}

pub async fn save_profile(
    backup_dir: PathBuf,
    saves_dir: PathBuf,
    output_file: PathBuf,
    output: Vec<u8>,
    existing_profile: Bl3Profile,
    new_profile: Bl3Profile,
    guardian_data_injection_required: bool,
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

    let backup_name = sanitize_filename::sanitize(backup_name);

    let (existing_profile_output, _) = existing_profile.as_bytes()?;

    tokio::fs::write(&backup_dir.join(backup_name), existing_profile_output).await?;

    info!("Saving profile: {}", new_profile.file_name);

    tokio::fs::write(output_file, output).await?;

    if guardian_data_injection_required {
        let guardian_rank = new_profile.profile_data.guardian_rank();

        let guardian_tokens = new_profile.profile_data.guardian_tokens();

        let guardian_rewards = new_profile.profile_data.guardian_rewards();

        state_mappers::inject_guardian_data_into_saves(
            backup_dir,
            saves_dir,
            guardian_rank,
            guardian_tokens,
            guardian_rewards,
        )
        .await?;
    }

    Ok(new_profile)
}

pub async fn load_files_after_save(
    saves_dir: PathBuf,
    file_saved: Bl3FileType,
) -> Result<(Bl3FileType, Vec<Bl3FileType>)> {
    let (_, all_files) = choose_save_directory::load_files_in_directory(saves_dir).await?;

    Ok((file_saved, all_files))
}
