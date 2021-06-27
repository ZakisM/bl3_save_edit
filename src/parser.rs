use anyhow::{Context, Result};

use crate::error::BL3ParserError;
use crate::error::ErrorExt;
use crate::models::CustomFormatData;

#[derive(Debug)]
pub enum FileType {
    PcSave,
    PcProfile,
}

const PC_SAVE_PREFIX_MAGIC: [u8; 32] = [
    0x71, 0x34, 0x36, 0xB3, 0x56, 0x63, 0x25, 0x5F, 0xEA, 0xE2, 0x83, 0x73, 0xF4, 0x98, 0xB8, 0x18, 0x2E, 0xE5, 0x42, 0x2E, 0x50, 0xA2, 0x0F, 0x49,
    0x87, 0x24, 0xE6, 0x65, 0x9A, 0xF0, 0x7C, 0xD7,
];

const PC_SAVE_XOR_MAGIC: [u8; 32] = [
    0x7C, 0x07, 0x69, 0x83, 0x31, 0x7E, 0x0C, 0x82, 0x5F, 0x2E, 0x36, 0x7F, 0x76, 0xB4, 0xA2, 0x71, 0x38, 0x2B, 0x6E, 0x87, 0x39, 0x05, 0x02, 0xC6,
    0xCD, 0xD8, 0xB1, 0xCC, 0xA1, 0x33, 0xF9, 0xB6,
];

const PC_PROFILE_PREFIX_MAGIC: [u8; 32] = [
    0xD8, 0x04, 0xB9, 0x08, 0x5C, 0x4E, 0x2B, 0xC0, 0x61, 0x9F, 0x7C, 0x8D, 0x5D, 0x34, 0x00, 0x56, 0xE7, 0x7B, 0x4E, 0xC0, 0xA4, 0xD6, 0xA7, 0x01,
    0x14, 0x15, 0xA9, 0x93, 0x1F, 0x27, 0x2C, 0x8F,
];

const PC_PROFILE_XOR_MAGIC: [u8; 32] = [
    0xE8, 0xDC, 0x3A, 0x66, 0xF7, 0xEF, 0x85, 0xE0, 0xBD, 0x4A, 0xA9, 0x73, 0x57, 0x99, 0x30, 0x8C, 0x94, 0x63, 0x59, 0xA8, 0xC9, 0xAE, 0xD9, 0x58,
    0x7D, 0x51, 0xB0, 0x1E, 0xBE, 0xD0, 0x77, 0x43,
];

pub fn read_header(i: &[u8]) -> nom::IResult<&[u8], &[u8], BL3ParserError<String>> {
    nom::bytes::complete::tag("GVAS")(i)
}

pub fn read_int(i: &[u8]) -> nom::IResult<&[u8], u32, BL3ParserError<String>> {
    nom::number::complete::le_u32(i)
}

pub fn read_short(i: &[u8]) -> nom::IResult<&[u8], u16, BL3ParserError<String>> {
    nom::number::complete::le_u16(i)
}

pub fn read_str(i: &[u8]) -> nom::IResult<&[u8], String, BL3ParserError<String>> {
    let (i, data_len) = read_int(i)?;
    let (i, res) = nom::bytes::complete::take(data_len)(i)?;

    let res = String::from_utf8(res[..res.len() - 1].to_vec()).parser_error()?;

    Ok((i, res))
}

pub fn read_custom_format_data(i: &[u8], fmt_count: u32) -> nom::IResult<&[u8], Vec<CustomFormatData>, BL3ParserError<String>> {
    let mut custom_format_data = Vec::with_capacity(fmt_count as usize);

    let mut i = i;

    for _ in 0..fmt_count {
        let (r, guid) = read_guid(i)?;
        let (r, entry) = read_int(r)?;

        custom_format_data.push(CustomFormatData {
            guid: format!("{:x?}", guid),
            entry,
        });

        i = r;
    }

    Ok((i, custom_format_data))
}

pub fn read_guid(i: &[u8]) -> nom::IResult<&[u8], &[u8], BL3ParserError<String>> {
    nom::bytes::complete::take(16_u32)(i)
}

pub fn decrypt<T: protobuf::Message>(data: &mut [u8], file_type: FileType) -> Result<T> {
    let prefix_magic = match file_type {
        FileType::PcSave => PC_SAVE_PREFIX_MAGIC,
        FileType::PcProfile => PC_PROFILE_PREFIX_MAGIC,
    };

    let xor_magic = match file_type {
        FileType::PcSave => PC_SAVE_XOR_MAGIC,
        FileType::PcProfile => PC_PROFILE_XOR_MAGIC,
    };

    for i in (0..data.len()).rev() {
        let b = if i < 32 {
            prefix_magic
                .get(i)
                .context("failed to decrypt save file, could not read PREFIX_MAGIC index")?
        } else {
            &data[i - 32]
        };

        data[i] ^= b ^ xor_magic
            .get(i % 32)
            .context("failed to decrypt save file, could not read XOR_MAGIC index")?;
    }

    let result: T = protobuf::Message::parse_from_bytes(data)?;

    Ok(result)
}
