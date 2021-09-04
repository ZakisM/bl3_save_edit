use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

use crate::bl3_ui::MessageResult;

const CONFIG_DIR: &str = "bl3_save_editor";
const CONFIG_NAME: &str = "config.toml";

#[derive(Debug, Clone)]
pub enum ConfigMessage {
    SaveCompleted(MessageResult<()>),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    config_dir: PathBuf,
    saves_dir: PathBuf,
}

impl Config {
    pub async fn open_dir() -> Result<()> {
        let config_dir = dirs::config_dir().context("Failed to read folder.")?;

        if config_dir.exists() {
            open::that(config_dir.join(CONFIG_DIR)).map_err(anyhow::Error::new)
        } else {
            bail!("Folder does not exist.")
        }
    }

    pub async fn load() -> Self {
        let config_dir = dirs::config_dir().unwrap_or_default().join(CONFIG_DIR);

        if let Ok(mut config) = tokio::fs::read(&config_dir.join(CONFIG_NAME))
            .await
            .map_err(anyhow::Error::new)
            .and_then(|c| toml::from_slice::<Config>(&c).map_err(anyhow::Error::new))
        {
            println!("Found existing config");

            //Set the config dir in case we ever want to change it from code
            config.config_dir = config_dir;

            config
        } else {
            println!("Creating default config");

            Self {
                config_dir,
                saves_dir: Default::default(),
            }
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

    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn saves_dir(&self) -> &PathBuf {
        &self.saves_dir
    }

    pub fn set_saves_dir(&mut self, dir: PathBuf) {
        self.saves_dir = dir;
    }
}
