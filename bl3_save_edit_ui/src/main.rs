use iced::{window, Application, Settings};

use crate::bl3_ui::Bl3Ui;

mod bl3_ui;
mod bl3_ui_style;
mod interaction;
mod resources;
mod views;
mod widgets;

fn main() {
    // let dialog = OpenSingleDir { dir: None };
    // let path = dialog.show().unwrap().unwrap();
    //
    // let data = fs::read(path).unwrap();

    // let test_files = Path::new("./bl3_save_edit_core/test_files");
    //
    // let mut dirs = tokio::fs::read_dir(test_files).await.unwrap();
    //
    // let mut all_data = vec![];
    //
    // while let Ok(entry) = dirs.next_entry().await {
    //     if let Some(entry) = entry {
    //         let path = entry.path();
    //         if !path.is_dir() {
    //             let data = tokio::fs::read(path).await.unwrap();
    //             all_data.push(data);
    //         }
    //     } else {
    //         break;
    //     }
    // }
    //
    // tokio_rayon::spawn(move || {
    //     all_data.par_iter().for_each(|l| {
    //         if let Err(e) = Bl3FileType::from_unknown_data(l) {
    //             eprintln!("{}", e);
    //         }
    //     })
    // })
    // .await;

    // let data = std::fs::read("./bl3_save_edit_core/test_files/69ps4_v103.sav").unwrap();
    // let bl3_file_type = Bl3FileType::from_unknown_data(&data).unwrap();

    let settings = Settings {
        window: window::Settings {
            min_size: Some((900, 400)),
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
