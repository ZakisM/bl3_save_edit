use iced::{window, Application, Settings};

use bl3_save_edit_core::bl3_save::Bl3Save;
use bl3_save_edit_core::parser::HeaderType;

use crate::bl3_ui::Bl3UiState;

mod bl3_ui;
mod bl3_ui_style;
mod interaction;
mod resources;
mod state_mappers;
mod views;
mod widgets;

fn main() {
    // let settings = Settings {
    //     window: window::Settings {
    //         min_size: Some((900, 800)),
    //         size: (1100, 800),
    //         ..window::Settings::default()
    //     },
    //     antialiasing: true,
    //     ..Settings::default()
    // };
    //
    // if let Err(e) = Bl3UiState::run(settings) {
    //     eprintln!("{:?}", e);
    // }
    let save_data = std::fs::read("./bl3_save_edit_core/test_files/19.sav").unwrap();
    let save = Bl3Save::from_bytes(&save_data, HeaderType::PcSave).unwrap();
}
