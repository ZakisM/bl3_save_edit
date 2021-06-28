use std::fmt;
use std::fmt::Formatter;

use anyhow::Result;

use crate::bl3_profile::profile_data::ProfileData;
use crate::file_helper;
use crate::file_helper::FileData;
use crate::models::CustomFormatData;
use crate::parser::{decrypt, FileType};
use crate::protos::oak_profile::Profile;

mod profile_currency;
mod profile_data;
mod util;

#[derive(Debug)]
pub struct Bl3Profile {
    profile_version: u32,
    package_version: u32,
    engine_major: u16,
    engine_minor: u16,
    engine_patch: u16,
    engine_build: u32,
    build_id: String,
    custom_format_version: u32,
    custom_format_data_count: u32,
    custom_format_data: Vec<CustomFormatData>,
    save_game_type: String,
    profile_data: ProfileData,
}

impl Bl3Profile {
    pub fn from_data(data: Vec<u8>, file_type: FileType) -> Result<Self> {
        let mut data = data;

        let FileData {
            file_version,
            package_version,
            engine_major,
            engine_minor,
            engine_patch,
            engine_build,
            build_id,
            custom_format_version,
            custom_format_data_count,
            custom_format_data,
            save_game_type,
            remaining_data,
        } = file_helper::read_file(&mut data)?;

        let profile: Profile = decrypt(remaining_data, file_type)?;

        let profile_data = ProfileData::from_profile(profile)?;

        Ok(Self {
            profile_version: file_version,
            package_version,
            engine_major,
            engine_minor,
            engine_patch,
            engine_build,
            build_id,
            custom_format_version,
            custom_format_data_count,
            custom_format_data,
            save_game_type,
            profile_data,
        })
    }
}

impl fmt::Display for Bl3Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Profile version: {}", self.profile_version)?;
        writeln!(f, "Package version: {}", self.package_version)?;
        writeln!(
            f,
            "Engine version: {}.{}.{}.{}",
            self.engine_major, self.engine_minor, self.engine_patch, self.engine_build
        )?;
        writeln!(f, "Build ID: {}", self.build_id)?;
        writeln!(f, "Custom Format Version: {}", self.custom_format_version)?;
        writeln!(f, "Custom Format Data Count: {}", self.custom_format_data_count)?;
        writeln!(f, "Savegame type: {}", self.save_game_type)?;

        Ok(())
    }
}
