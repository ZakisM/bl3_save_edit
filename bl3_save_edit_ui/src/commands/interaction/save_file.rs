use std::path::PathBuf;

use anyhow::Result;

pub async fn save_file(output_file: PathBuf, output: Vec<u8>) -> Result<()> {
    tokio::fs::write(output_file, output).await?;

    Ok(())
}
