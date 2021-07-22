use anyhow::{Context, Result};
use byteorder::{LittleEndian, WriteBytesExt};
use strum::Display;

use crate::error::BL3ParserError;
use crate::error::ErrorExt;
use crate::models::CustomFormatData;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Display)]
pub enum HeaderType {
    #[strum(to_string = "PC Save")]
    PcSave,
    #[strum(to_string = "PC Profile")]
    PcProfile,
    #[strum(to_string = "PS4 Save")]
    Ps4Save,
    #[strum(to_string = "PS4 Profile")]
    Ps4Profile,
}

impl std::default::Default for HeaderType {
    fn default() -> Self {
        Self::PcSave
    }
}

impl HeaderType {
    pub const SAVE_TYPES: [HeaderType; 2] = [HeaderType::PcSave, HeaderType::Ps4Save];

    pub const PROFILE_TYPES: [HeaderType; 2] = [HeaderType::PcProfile, HeaderType::Ps4Profile];
}

const PC_SAVE_PREFIX_MAGIC: [u8; 32] = [
    0x71, 0x34, 0x36, 0xB3, 0x56, 0x63, 0x25, 0x5F, 0xEA, 0xE2, 0x83, 0x73, 0xF4, 0x98, 0xB8, 0x18,
    0x2E, 0xE5, 0x42, 0x2E, 0x50, 0xA2, 0x0F, 0x49, 0x87, 0x24, 0xE6, 0x65, 0x9A, 0xF0, 0x7C, 0xD7,
];

const PC_SAVE_XOR_MAGIC: [u8; 32] = [
    0x7C, 0x07, 0x69, 0x83, 0x31, 0x7E, 0x0C, 0x82, 0x5F, 0x2E, 0x36, 0x7F, 0x76, 0xB4, 0xA2, 0x71,
    0x38, 0x2B, 0x6E, 0x87, 0x39, 0x05, 0x02, 0xC6, 0xCD, 0xD8, 0xB1, 0xCC, 0xA1, 0x33, 0xF9, 0xB6,
];

const PC_PROFILE_PREFIX_MAGIC: [u8; 32] = [
    0xD8, 0x04, 0xB9, 0x08, 0x5C, 0x4E, 0x2B, 0xC0, 0x61, 0x9F, 0x7C, 0x8D, 0x5D, 0x34, 0x00, 0x56,
    0xE7, 0x7B, 0x4E, 0xC0, 0xA4, 0xD6, 0xA7, 0x01, 0x14, 0x15, 0xA9, 0x93, 0x1F, 0x27, 0x2C, 0x8F,
];

const PC_PROFILE_XOR_MAGIC: [u8; 32] = [
    0xE8, 0xDC, 0x3A, 0x66, 0xF7, 0xEF, 0x85, 0xE0, 0xBD, 0x4A, 0xA9, 0x73, 0x57, 0x99, 0x30, 0x8C,
    0x94, 0x63, 0x59, 0xA8, 0xC9, 0xAE, 0xD9, 0x58, 0x7D, 0x51, 0xB0, 0x1E, 0xBE, 0xD0, 0x77, 0x43,
];

const PS4_SAVE_PREFIX_MAGIC: [u8; 32] = [
    0xd1, 0x7b, 0xbf, 0x75, 0x4c, 0xc1, 0x80, 0x30, 0x37, 0x92, 0xbd, 0xd0, 0x18, 0x3e, 0x4a, 0x5f,
    0x43, 0xa2, 0x46, 0xa0, 0xed, 0xdb, 0x2d, 0x9f, 0x56, 0x5f, 0x8b, 0x3d, 0x6e, 0x73, 0xe6, 0xb8,
];

const PS4_SAVE_XOR_MAGIC: [u8; 32] = [
    0xfb, 0xfd, 0xfd, 0x51, 0x3a, 0x5c, 0xdb, 0x20, 0xbb, 0x5e, 0xc7, 0xaf, 0x66, 0x6f, 0xb6, 0x9a,
    0x9a, 0x52, 0x67, 0x0f, 0x19, 0x5d, 0xd3, 0x84, 0x15, 0x19, 0xc9, 0x4a, 0x79, 0x67, 0xda, 0x6d,
];

const PS4_PROFILE_PREFIX_MAGIC: [u8; 32] = [
    0xad, 0x1e, 0x60, 0x4e, 0x42, 0x9e, 0xa9, 0x33, 0xb2, 0xf5, 0x01, 0xe1, 0x02, 0x4d, 0x08, 0x75,
    0xb1, 0xad, 0x1a, 0x3d, 0xa1, 0x03, 0x6b, 0x1a, 0x17, 0xe6, 0xec, 0x0f, 0x60, 0x8d, 0xb4, 0xf9,
];

const PS4_PROFILE_XOR_MAGIC: [u8; 32] = [
    0xba, 0x0e, 0x86, 0x1d, 0x58, 0xe1, 0x92, 0x21, 0x30, 0xd6, 0xcb, 0xf0, 0xd0, 0x82, 0xd5, 0x58,
    0x36, 0x12, 0xe1, 0xf6, 0x39, 0x44, 0x88, 0xea, 0x4e, 0xfb, 0x04, 0x74, 0x07, 0x95, 0x3a, 0xa2,
];

pub fn read_header(i: &[u8]) -> nom::IResult<&[u8], &[u8], BL3ParserError<String>> {
    nom::bytes::complete::tag("GVAS")(i)
}

pub fn read_int(i: &[u8]) -> nom::IResult<&[u8], u32, BL3ParserError<String>> {
    nom::number::complete::le_u32(i)
}

pub fn read_be_signed_int(i: &[u8]) -> nom::IResult<&[u8], i32, BL3ParserError<String>> {
    nom::number::complete::be_i32(i)
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

pub fn write_str<T: std::io::Write>(output: &mut T, s: &str) -> Result<()> {
    let data_len = s.len();

    if s.is_empty() {
        output.write_u32::<LittleEndian>(1)?;
    } else {
        output.write_u32::<LittleEndian>((data_len + 1) as u32)?;
        output.write_all(s.as_bytes())?;
        output.write_all(b"\x00")?;
    }

    Ok(())
}

pub fn read_custom_format_data(
    i: &[u8],
    fmt_count: u32,
) -> nom::IResult<&[u8], Vec<CustomFormatData>, BL3ParserError<String>> {
    let mut custom_format_data = Vec::with_capacity(fmt_count as usize);

    let mut i = i;

    for _ in 0..fmt_count {
        let (r, guid) = read_guid(i)?;
        let (r, entry) = read_int(r)?;

        custom_format_data.push(CustomFormatData {
            guid: guid.to_vec(),
            entry,
        });

        i = r;
    }

    Ok((i, custom_format_data))
}

pub fn read_guid(i: &[u8]) -> nom::IResult<&[u8], &[u8], BL3ParserError<String>> {
    nom::bytes::complete::take(16_u32)(i)
}

pub fn decrypt<T: protobuf::Message>(data: &[u8], header_type: &HeaderType) -> Result<T> {
    let (prefix_magic, xor_magic) = match header_type {
        HeaderType::PcSave => (PC_SAVE_PREFIX_MAGIC, PC_SAVE_XOR_MAGIC),
        HeaderType::PcProfile => (PC_PROFILE_PREFIX_MAGIC, PC_PROFILE_XOR_MAGIC),
        HeaderType::Ps4Save => (PS4_SAVE_PREFIX_MAGIC, PS4_SAVE_XOR_MAGIC),
        HeaderType::Ps4Profile => (PS4_PROFILE_PREFIX_MAGIC, PS4_PROFILE_XOR_MAGIC),
    };

    // Clone data so we can decrypt multiple times (when we don't know save type)
    let mut data = data.to_vec();
    let data = data.as_mut_slice();

    for i in (0..data.len()).rev() {
        let b = if i < 32 {
            prefix_magic.get(i).with_context(|| {
                format!(
                    "failed to decrypt save file, could not read PREFIX_MAGIC index for: {:?}",
                    header_type
                )
            })?
        } else {
            &data[i - 32]
        };

        data[i] ^= b ^ xor_magic.get(i % 32).with_context(|| {
            format!(
                "failed to decrypt save file, could not read XOR_MAGIC index for: {:?}",
                header_type
            )
        })?;
    }

    let result: T = protobuf::Message::parse_from_bytes(data)?;

    Ok(result)
}

pub fn encrypt(data: &mut [u8], header_type: HeaderType) -> Result<()> {
    let (prefix_magic, xor_magic) = match header_type {
        HeaderType::PcSave => (PC_SAVE_PREFIX_MAGIC, PC_SAVE_XOR_MAGIC),
        HeaderType::PcProfile => (PC_PROFILE_PREFIX_MAGIC, PC_PROFILE_XOR_MAGIC),
        HeaderType::Ps4Save => (PS4_SAVE_PREFIX_MAGIC, PS4_SAVE_XOR_MAGIC),
        HeaderType::Ps4Profile => (PS4_PROFILE_PREFIX_MAGIC, PS4_PROFILE_XOR_MAGIC),
    };

    for i in 0..data.len() {
        let b = if i < 32 {
            prefix_magic.get(i).with_context(|| {
                format!(
                    "failed to encrypt save file, could not read PREFIX_MAGIC index for: {:?}",
                    header_type
                )
            })?
        } else {
            &data[i - 32]
        };

        data[i] ^= b ^ xor_magic.get(i % 32).with_context(|| {
            format!(
                "failed to encrypt save file, could not read XOR_MAGIC index for: {:?}",
                header_type
            )
        })?;
    }

    Ok(())
}
