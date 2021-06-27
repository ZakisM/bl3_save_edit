use anyhow::{bail, Result};
use nom::Finish;

use crate::models::CustomFormatData;
use crate::parser::{read_custom_format_data, read_header, read_int, read_short, read_str};

#[derive(Debug)]
pub struct FileData<'a> {
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
    pub remaining_data: &'a mut [u8],
}

pub fn read_file(data: &mut [u8]) -> Result<FileData> {
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

    let remaining_data = &mut data[data_read..];

    if remaining_data.len() != remaining_data_len as usize {
        bail!("failed to parse the first part of the file");
    }

    Ok(FileData {
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
