use iced::{window, Application, Settings};

use crate::bl3_ui::Bl3Ui;

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
        ..Settings::default()
    };

    if let Err(e) = Bl3Ui::run(settings) {
        eprintln!("{:?}", e);
    }
}
