use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::bl3_ui::MessageResult;

const CONFIG_DIR: &str = "bl3-save-editor";
const CONFIG_NAME: &str = "config.toml";

#[derive(Debug, Clone)]
pub enum ConfigMessage {
    SaveCompleted(MessageResult<()>),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    saves_dir: PathBuf,
}

impl Config {
    pub async fn load() -> Self {
        let existing_config_path = dirs::config_dir()
            .unwrap_or_default()
            .join(CONFIG_DIR)
            .join(CONFIG_NAME);

        if let Ok(config) = tokio::fs::read(existing_config_path)
            .await
            .map_err(anyhow::Error::new)
            .and_then(|c| toml::from_slice(&c).map_err(anyhow::Error::new))
        {
            config
        } else {
            Self::default()
        }
    }

    pub async fn save(self) -> Result<()> {
        println!("Saving config...");

        let config_dir = dirs::config_dir().unwrap_or_default().join(CONFIG_DIR);

        if !config_dir.exists() {
            tokio::fs::create_dir(&config_dir).await?;
        }

        let output = toml::to_vec(&self)?;

        let mut config_file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_dir.join(CONFIG_NAME))
            .await?;

        config_file.write_all(&output).await?;

        Ok(())
    }

    pub fn saves_dir(&self) -> &PathBuf {
        &self.saves_dir
    }

    pub fn set_saves_dir(&mut self, dir: PathBuf) {
        self.saves_dir = dir;
    }
}
