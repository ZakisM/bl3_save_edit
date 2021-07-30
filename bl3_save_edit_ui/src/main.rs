use iced::{window, Application, Settings};

use crate::bl3_ui::Bl3UiState;

mod bl3_ui;
mod bl3_ui_style;
mod commands;
mod resources;
mod state_mappers;
mod views;
mod widgets;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let settings = Settings {
        window: window::Settings {
            min_size: Some((1650, 800)),
            size: (1650, 800),
            ..window::Settings::default()
        },
        antialiasing: true,
        text_multithreading: true,
        ..Settings::default()
    };

    if let Err(e) = Bl3UiState::run(settings) {
        eprintln!("{:?}", e);
    }
}
