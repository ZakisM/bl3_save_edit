use std::path::PathBuf;

use anyhow::Result;

use bl3_save_edit_core::bl3_profile::Bl3Profile;
use bl3_save_edit_core::bl3_save::Bl3Save;

pub async fn save_file(output_file: PathBuf, output: Vec<u8>, save: Bl3Save) -> Result<Bl3Save> {
    println!("Saving file: {}", save.file_name);

    tokio::fs::write(output_file, output).await?;

    Ok(save)
}

pub async fn save_profile(
    output_file: PathBuf,
    output: Vec<u8>,
    profile: Bl3Profile,
) -> Result<Bl3Profile> {
    println!("Saving profile: {}", profile.file_name);

    tokio::fs::write(output_file, output).await?;

    Ok(profile)
}
