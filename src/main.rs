use std::fs;

use anyhow::Result;

use crate::bl3_profile::Bl3Profile;
use crate::bl3_save::Bl3Save;
use crate::parser::FileType;

mod bl3_profile;
mod bl3_save;
mod error;
mod file_helper;
mod game_data;
mod models;
mod parser;
mod protos;

fn main() -> Result<()> {
    let profile_file_data = fs::read("./test_files/profile.sav")?;
    let bl3_profile = Bl3Profile::from_data(profile_file_data, FileType::PcProfile)?;

    println!("{}", bl3_profile);

    // let save_file_data = fs::read("./test_files/19.sav")?;
    // let bl3_save = Bl3Save::from_data(save_file_data, FileType::PcSave)?;

    // println!("{}", bl3_save);

    Ok(())
}
