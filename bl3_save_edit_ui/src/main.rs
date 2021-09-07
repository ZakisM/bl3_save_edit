#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use anyhow::Result;
use iced::{window, Application, Settings};
use tracing::error;

use crate::bl3_ui::Bl3Application;
use crate::config::Bl3Config;

mod bl3_ui;
mod bl3_ui_style;
mod commands;
mod config;
mod resources;
mod state_mappers;
mod views;
mod widgets;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    env::set_var("RUST_LOG", "INFO");

    let config = Bl3Config::load();

    let logs_dir = config.config_dir().join("logs");

    if !logs_dir.exists() {
        std::fs::create_dir_all(&logs_dir)?;
    }

    let file_appender = tracing_appender::rolling::daily(logs_dir, "bl3_save_editor.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt().with_writer(non_blocking).init();

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
