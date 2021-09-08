#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use anyhow::{Context, Result};
use iced::{window, Application, Settings};
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
mod views;
mod widgets;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    let mut pargs = pico_args::Arguments::from_env();

    env::set_var("RUST_LOG", "INFO");

    let config = Bl3Config::load();

    let logs_dir = config.config_dir().join("logs");

    if !logs_dir.exists() {
        std::fs::create_dir_all(&logs_dir)?;
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

    let settings = Settings {
        flags: config,
        window: window::Settings {
            min_size: Some((1650, 800)),
            size: (1650, 800),
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
