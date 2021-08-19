use std::fmt::Formatter;
use std::path::Path;

use anyhow::{bail, Context, Result};
use nom::Finish;

use crate::bl3_profile::Bl3Profile;
use crate::bl3_save::Bl3Save;
use crate::models::CustomFormatData;
use crate::parser::{
    read_custom_format_data, read_header, read_int, read_short, read_str, HeaderType,
};

#[derive(Debug, Clone)]
pub struct FileData<'a> {
    pub file_name: &'a str,
    pub file_version: u32,
    pub package_version: u32,
    pub engine_major: u16,
    pub engine_minor: u16,
    pub engine_patch: u16,
    pub engine_build: u32,
    pub build_id: String,
    pub custom_format_version: u32,
    pub custom_format_data_count: u32,
    pub custom_format_data: Vec<CustomFormatData>,
    pub save_game_type: String,
    pub remaining_data: &'a [u8],
}

pub fn read_bytes<'a>(file_name: &'a str, data: &'a [u8]) -> Result<FileData<'a>> {
    let (r, _) = read_header(data).finish()?;
    let (r, file_version) = read_int(r).finish()?;
    let (r, package_version) = read_int(r).finish()?;
    let (r, engine_major) = read_short(r).finish()?;
    let (r, engine_minor) = read_short(r).finish()?;
    let (r, engine_patch) = read_short(r).finish()?;
    let (r, engine_build) = read_int(r).finish()?;
    let (r, build_id) = read_str(r).finish()?;
    let (r, custom_format_version) = read_int(r).finish()?;
    let (r, custom_format_data_count) = read_int(r).finish()?;
    let (r, custom_format_data) = read_custom_format_data(r, custom_format_data_count).finish()?;
    let (r, save_game_type) = read_str(r).finish()?;
    let (r, remaining_data_len) = read_int(r).finish()?;

    let data_read = data.len() - r.len();

    let remaining_data = &data[data_read..];

    if remaining_data.len() != remaining_data_len as usize {
        bail!("failed to parse the first part of the file")
    }

    Ok(FileData {
        file_name,
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
    })
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub enum Bl3FileType {
    PcSave(Bl3Save),
    PcProfile(Bl3Profile),
    Ps4Save(Bl3Save),
    Ps4Profile(Bl3Profile),
}

impl std::default::Default for Bl3FileType {
    fn default() -> Self {
        Self::PcSave(Bl3Save::default())
    }
}

impl std::fmt::Display for Bl3FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bl3FileType::PcSave(save) | Bl3FileType::Ps4Save(save) => write!(
                f,
                "[{}] {} ({}) - Level {}",
                save.header_type,
                save.character_data.character.preferred_character_name,
                save.character_data.player_class(),
                save.character_data.player_level()
            ),
            Bl3FileType::PcProfile(profile) | Bl3FileType::Ps4Profile(profile) => {
                write!(
                    f,
                    "[{}] Golden Keys: {}",
                    profile.header_type, profile.profile_data.golden_keys,
                )
            }
        }
    }
}

impl Bl3FileType {
    pub fn from_unknown_data(file_name: &Path, data: &[u8]) -> Result<Bl3FileType> {
        let file_name = file_name
            .to_str()
            .with_context(|| format!("failed to read file name: {:?}", file_name.file_name()))?;
        let file_data = read_bytes(file_name, data)?;

        if let Ok(save) = Bl3Save::from_file_data(&file_data, HeaderType::PcSave) {
            Ok(Bl3FileType::PcSave(save))
        } else if let Ok(profile) = Bl3Profile::from_file_data(&file_data, HeaderType::PcProfile) {
            Ok(Bl3FileType::PcProfile(profile))
        } else if let Ok(save) = Bl3Save::from_file_data(&file_data, HeaderType::Ps4Save) {
            Ok(Bl3FileType::Ps4Save(save))
        } else if let Ok(profile) = Bl3Profile::from_file_data(&file_data, HeaderType::Ps4Profile) {
            Ok(Bl3FileType::Ps4Profile(profile))
        } else {
            bail!("could not recognize file type")
        }
    }
}
