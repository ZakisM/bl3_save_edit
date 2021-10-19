#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use anyhow::{bail, Context, Result};
use iced::window::icon::Icon;
use iced::{window, Application, Settings};
use image::{GenericImageView, ImageFormat};
use tracing::{error, info};

use crate::bl3_ui::Bl3Application;
use crate::config::Bl3Config;
use crate::update::remove_file;

mod bl3_ui;
mod bl3_ui_style;
mod commands;
mod config;
mod resources;
mod state_mappers;
mod update;
mod util;
mod views;
mod widgets;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const WINDOW_ICON: &[u8] = include_bytes!("../../build_resources/windows/win_bl3_save_edit.ico");

fn main() -> Result<()> {
    let mut pargs = pico_args::Arguments::from_env();

    env::set_var("RUST_LOG", "INFO");

    let config = Bl3Config::load();

    let logs_dir = config.config_dir().join("logs");
    let backups_dir = config.config_dir().join("backups");

    if !logs_dir.exists() {
        std::fs::create_dir_all(&logs_dir)?;
    }

    if !backups_dir.exists() {
        std::fs::create_dir_all(&backups_dir)?;
    }

    let file_appender = tracing_appender::rolling::daily(logs_dir, "bl3_save_editor.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt().with_writer(non_blocking).init();

    let previous_update_cleanup_path: Result<String> = pargs
        .value_from_str("--cleanup_previous_path")
        .context("No previous update path passed so ignoring it.");

    match previous_update_cleanup_path {
        Ok(p) => {
            info!("Cleaning up previous update file: {}", p);
            std::thread::spawn(move || remove_file(&p));
        }
        Err(e) => {
            info!("Skipping post-update cleanup: {}", e);
        }
    }

    let window_icon = match image::load_from_memory_with_format(WINDOW_ICON, ImageFormat::Ico)
        .map(|i| (i.to_rgba8().into_raw(), i.width(), i.height()))
        .map_err(anyhow::Error::new)
        .and_then(|(i, width, height)| {
            Icon::from_rgba(i, width, height).map_err(anyhow::Error::new)
        }) {
        Ok(icon) => icon,
        Err(e) => {
            let msg = format!("Failed to load window_icon: {}", e);

            error!("{}", msg);
            bail!("{}", msg)
        }
    };

    let settings = Settings {
        flags: config,
        window: window::Settings {
            min_size: Some((1320, 750)),
            size: (1650, 800),
            icon: Some(window_icon),
            ..window::Settings::default()
        },
        antialiasing: true,
        text_multithreading: true,
        ..Settings::default()
    };

    if let Err(e) = Bl3Application::run(settings) {
        error!("{}", e);
    }

    Ok(())
}
