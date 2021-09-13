use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tracing::info;

use crate::bl3_ui::MessageResult;

const CONFIG_DIR: &str = "bl3_save_editor";
const CONFIG_NAME: &str = "config.toml";

#[derive(Debug, Clone)]
pub enum ConfigMessage {
    SaveCompleted(MessageResult<()>),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Bl3Config {
    config_dir: PathBuf,
    saves_dir: PathBuf,
    #[serde(default = "default_scale_factor")]
    ui_scale_factor: f64,
}

fn default_scale_factor() -> f64 {
    1.0
}

impl Bl3Config {
    pub async fn open_dir() -> Result<()> {
        let config_dir = dirs::config_dir().context("Failed to read folder.")?;

        if config_dir.exists() {
            open::that(config_dir.join(CONFIG_DIR)).map_err(anyhow::Error::new)
        } else {
            bail!("Folder does not exist.")
        }
    }

    pub fn load() -> Self {
        let config_dir = dirs::config_dir().unwrap_or_default().join(CONFIG_DIR);

        if let Ok(mut config) = std::fs::read(&config_dir.join(CONFIG_NAME))
            .map_err(anyhow::Error::new)
            .and_then(|c| toml::from_slice::<Bl3Config>(&c).map_err(anyhow::Error::new))
        {
            info!("Found existing config");

            //Set the config dir in case we ever want to change it from code
            config.config_dir = config_dir;

            config
        } else {
            info!("Creating default config");

            Self {
                config_dir,
                saves_dir: Default::default(),
                ui_scale_factor: default_scale_factor(),
            }
        }
    }

    pub async fn save(self) -> Result<()> {
        info!("Saving config...");

        let config_dir = dirs::config_dir().unwrap_or_default().join(CONFIG_DIR);

        if !config_dir.exists() {
            tokio::fs::create_dir_all(&config_dir).await?;
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

    pub fn ui_scale_factor(&self) -> f64 {
        self.ui_scale_factor
    }

    pub fn set_ui_scale_factor(&mut self, ui_scale_factor: f64) {
        self.ui_scale_factor = ui_scale_factor;
    }
}
