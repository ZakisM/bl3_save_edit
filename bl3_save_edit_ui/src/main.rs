use iced::{window, Application, Settings};

use crate::bl3_ui::Bl3UiState;

mod bl3_ui;
mod bl3_ui_style;
mod interaction;
mod resources;
mod state_mappers;
mod views;
mod widgets;

fn main() {
    let settings = Settings {
        window: window::Settings {
            min_size: Some((900, 800)),
            size: (1100, 800),
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
