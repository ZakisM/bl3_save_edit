use std::fmt;
use std::fmt::Formatter;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use byteorder::{LittleEndian, WriteBytesExt};

use crate::bl3_profile::profile_data::ProfileData;
use crate::file_helper::FileData;
use crate::game_data::{
    PROFILE_ECHO_THEMES, PROFILE_ECHO_THEMES_DEFAULTS, PROFILE_EMOTES, PROFILE_EMOTES_DEFAULTS,
    PROFILE_HEADS, PROFILE_HEADS_DEFAULTS, PROFILE_ROOM_DECORATIONS, PROFILE_SKINS,
    PROFILE_SKINS_DEFAULTS, PROFILE_WEAPON_SKINS, PROFILE_WEAPON_TRINKETS,
};
use crate::models::CustomFormatData;
use crate::parser::{decrypt, encrypt, HeaderType};
use crate::protos::oak_profile::Profile;
use crate::{file_helper, parser};

pub mod guardian_reward;
pub mod profile_currency;
pub mod profile_data;
pub mod science_levels;
pub mod sdu;
pub mod skins;
pub mod util;

#[derive(Debug, Clone, Default, Eq, Ord, PartialOrd)]
pub struct Bl3Profile {
    pub file_name: String,
    pub save_game_version: u32,
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
    pub header_type: HeaderType,
    pub profile_data: ProfileData,
}

impl std::cmp::PartialEq for Bl3Profile {
    fn eq(&self, other: &Self) -> bool {
        self.profile_data == other.profile_data
    }
}

impl Bl3Profile {
    pub fn from_file_data(file_data: &FileData, header_type: HeaderType) -> Result<Self> {
        let remaining_data = file_data.remaining_data;

        let profile: Profile = decrypt(remaining_data, &header_type)?;

        let profile_data = ProfileData::from_profile(profile)?;

        let FileData {
            file_location,
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
            ..
        } = file_data.clone();

        let file_name = file_location
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .context("failed to read file name")?;

        Ok(Self {
            file_name,
            save_game_version: file_version,
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
            header_type,
            profile_data,
        })
    }

    pub fn from_bytes(file_location: &Path, data: &[u8], header_type: HeaderType) -> Result<Self> {
        let file_data = file_helper::read_bytes(file_location, data)?;

        Self::from_file_data(&file_data, header_type)
    }

    pub fn as_bytes(&self) -> Result<(Vec<u8>, Bl3Profile)> {
        let mut output = Vec::new();

        output.write_all(b"GVAS")?;
        output.write_u32::<LittleEndian>(self.save_game_version)?;
        output.write_u32::<LittleEndian>(self.package_version)?;
        output.write_u16::<LittleEndian>(self.engine_major)?;
        output.write_u16::<LittleEndian>(self.engine_minor)?;
        output.write_u16::<LittleEndian>(self.engine_patch)?;
        output.write_u32::<LittleEndian>(self.engine_build)?;
        parser::write_str(&mut output, &self.build_id)?;
        output.write_u32::<LittleEndian>(self.custom_format_version)?;
        output.write_u32::<LittleEndian>(self.custom_format_data_count)?;

        for cfd in &self.custom_format_data {
            output.write_all(&cfd.guid)?;
            output.write_u32::<LittleEndian>(cfd.entry)?;
        }

        parser::write_str(&mut output, &self.save_game_type)?;

        let mut data = protobuf::Message::write_to_bytes(&self.profile_data.profile)?;

        encrypt(&mut data, self.header_type)?;

        output.write_u32::<LittleEndian>(data.len() as u32)?;
        output.append(&mut data);

        //Now try re-reading it also - there's no point making an invalid save
        let file_name = Path::new(&self.file_name);
        let new_profile = Self::from_bytes(file_name, &output, self.header_type)?;

        Ok((output, new_profile))
    }
}

impl fmt::Display for Bl3Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Profile version: {}", self.save_game_version)?;
        writeln!(f, "Package version: {}", self.package_version)?;
        writeln!(
            f,
            "Engine version: {}.{}.{}.{}",
            self.engine_major, self.engine_minor, self.engine_patch, self.engine_build
        )?;
        writeln!(f, "Build ID: {}", self.build_id)?;
        writeln!(f, "Custom Format Version: {}", self.custom_format_version)?;
        writeln!(
            f,
            "Custom Format Data Count: {}",
            self.custom_format_data_count
        )?;
        writeln!(f, "Savegame type: {}", self.save_game_type)?;

        writeln!(f, "Keys:")?;
        writeln!(
            f,
            "{:>1}- Golden Keys: {}",
            " ",
            self.profile_data.golden_keys()
        )?;
        writeln!(
            f,
            "{:>1}- Diamond Keys: {}",
            " ",
            self.profile_data.diamond_keys()
        )?;
        writeln!(
            f,
            "{:>1}- Vault Card 1 Keys: {}",
            " ",
            self.profile_data.vault_card_1_keys()
        )?;
        writeln!(
            f,
            "{:>1}- Vault Card 1 Chests: {}",
            " ",
            self.profile_data.vault_card_1_chests()
        )?;
        writeln!(f, "Guardian Rank: {}", self.profile_data.guardian_rank())?;
        writeln!(
            f,
            "Guardian Rank Tokens: {}",
            self.profile_data.guardian_tokens()
        )?;
        writeln!(
            f,
            "Borderlands Science Level: {} ({} solved)",
            self.profile_data.borderlands_science_info().science_level,
            self.profile_data.borderlands_science_info().solves
        )?;
        writeln!(
            f,
            "Borderlands Science Tokens: {}",
            self.profile_data.borderlands_science_info().tokens
        )?;

        writeln!(f, "SDUs:")?;

        for slot in self.profile_data.sdu_slots() {
            writeln!(f, "{:>1}- {}: {}/{}", " ", slot.sdu, slot.current, slot.max)?;
        }

        writeln!(f, "Items in Bank: {}", self.profile_data.bank_items().len())?;
        writeln!(
            f,
            "Items in Lost Loot machine: {}",
            self.profile_data.lost_loot_items().len()
        )?;

        writeln!(
            f,
            "Character Skins Unlocked: {}/{}",
            self.profile_data.character_skins_unlocked(),
            PROFILE_SKINS.len() + PROFILE_SKINS_DEFAULTS.len()
        )?;
        writeln!(
            f,
            "Character Heads Unlocked: {}/{}",
            self.profile_data.character_heads_unlocked(),
            PROFILE_HEADS.len() + PROFILE_HEADS_DEFAULTS.len()
        )?;
        writeln!(
            f,
            "ECHO Themes Unlocked: {}/{}",
            self.profile_data.echo_themes_unlocked(),
            PROFILE_ECHO_THEMES.len() + PROFILE_ECHO_THEMES_DEFAULTS.len()
        )?;
        writeln!(
            f,
            "Emotes Unlocked: {}/{}",
            self.profile_data.profile_emotes_unlocked(),
            PROFILE_EMOTES.len() + PROFILE_EMOTES_DEFAULTS.len()
        )?;
        writeln!(
            f,
            "Room Decorations Unlocked: {}/{}",
            self.profile_data.room_decorations_unlocked(),
            PROFILE_ROOM_DECORATIONS.len()
        )?;
        writeln!(
            f,
            "Weapon Skins Unlocked: {}/{}",
            self.profile_data.weapon_skins_unlocked(),
            PROFILE_WEAPON_SKINS.len()
        )?;
        writeln!(
            f,
            "Weapon Trinkets Unlocked: {}/{}",
            self.profile_data.weapon_trinkets_unlocked(),
            PROFILE_WEAPON_TRINKETS.len()
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::bl3_profile::science_levels::BorderlandsScienceLevel;
    use crate::bl3_profile::sdu::{ProfileSduSlot, ProfileSduSlotData};

    use super::*;

    #[test]
    fn test_from_data_pc_1() {
        let filename = Path::new("./test_files/1prof.sav");

        let mut profile_file_data = fs::read(&filename).expect("failed to read test_file");

        let bl3_profile =
            Bl3Profile::from_bytes(filename, &mut profile_file_data, HeaderType::PcProfile)
                .expect("failed to read test profile");

        assert_eq!(bl3_profile.profile_data.golden_keys(), 23);
        assert_eq!(bl3_profile.profile_data.diamond_keys(), 0);
        assert_eq!(bl3_profile.profile_data.vault_card_1_keys(), 0);
        assert_eq!(bl3_profile.profile_data.vault_card_1_chests(), 0);
        assert_eq!(bl3_profile.profile_data.guardian_rank(), 226);
        assert_eq!(bl3_profile.profile_data.guardian_tokens(), 8);
        assert_eq!(
            bl3_profile
                .profile_data
                .borderlands_science_info()
                .science_level,
            BorderlandsScienceLevel::Claptrap
        );
        assert_eq!(
            bl3_profile.profile_data.borderlands_science_info().solves,
            0
        );
        assert_eq!(
            bl3_profile.profile_data.borderlands_science_info().tokens,
            0
        );
        assert_eq!(
            *bl3_profile.profile_data.sdu_slots(),
            vec![
                ProfileSduSlotData {
                    sdu: ProfileSduSlot::Bank,
                    current: 23,
                    max: ProfileSduSlot::Bank.maximum(),
                },
                ProfileSduSlotData {
                    sdu: ProfileSduSlot::LostLoot,
                    current: 8,
                    max: ProfileSduSlot::LostLoot.maximum(),
                },
            ]
        );

        assert_eq!(bl3_profile.profile_data.bank_items().len(), 399);
        assert_eq!(bl3_profile.profile_data.lost_loot_items().len(), 13);
        assert_eq!(bl3_profile.profile_data.character_skins_unlocked(), 204);
        assert_eq!(bl3_profile.profile_data.character_heads_unlocked(), 136);
        assert_eq!(bl3_profile.profile_data.echo_themes_unlocked(), 55);
        assert_eq!(bl3_profile.profile_data.profile_emotes_unlocked(), 72);
        assert_eq!(bl3_profile.profile_data.room_decorations_unlocked(), 93);
        assert_eq!(bl3_profile.profile_data.weapon_skins_unlocked(), 23);
        assert_eq!(bl3_profile.profile_data.weapon_trinkets_unlocked(), 63);
    }

    #[test]
    fn test_from_data_pc_2() {
        let filename = Path::new("./test_files/profile.sav");

        let mut profile_file_data = fs::read(&filename).expect("failed to read test_file");

        let bl3_profile =
            Bl3Profile::from_bytes(filename, &mut profile_file_data, HeaderType::PcProfile)
                .expect("failed to read test profile");

        assert_eq!(bl3_profile.profile_data.golden_keys(), 1);
        assert_eq!(bl3_profile.profile_data.diamond_keys(), 0);
        assert_eq!(bl3_profile.profile_data.vault_card_1_keys(), 0);
        assert_eq!(bl3_profile.profile_data.vault_card_1_chests(), 0);
        assert_eq!(bl3_profile.profile_data.guardian_rank(), 200);
        assert_eq!(bl3_profile.profile_data.guardian_tokens(), 0);
        assert_eq!(
            bl3_profile
                .profile_data
                .borderlands_science_info()
                .science_level,
            BorderlandsScienceLevel::None
        );
        assert_eq!(
            bl3_profile.profile_data.borderlands_science_info().solves,
            0
        );
        assert_eq!(
            bl3_profile.profile_data.borderlands_science_info().tokens,
            0
        );
        assert_eq!(
            *bl3_profile.profile_data.sdu_slots(),
            vec![
                ProfileSduSlotData {
                    sdu: ProfileSduSlot::Bank,
                    current: 8,
                    max: ProfileSduSlot::Bank.maximum(),
                },
                ProfileSduSlotData {
                    sdu: ProfileSduSlot::LostLoot,
                    current: 8,
                    max: ProfileSduSlot::LostLoot.maximum(),
                },
            ]
        );

        assert_eq!(bl3_profile.profile_data.bank_items().len(), 0);
        assert_eq!(bl3_profile.profile_data.lost_loot_items().len(), 13);
        assert_eq!(bl3_profile.profile_data.character_skins_unlocked(), 27);
        assert_eq!(bl3_profile.profile_data.character_heads_unlocked(), 22);
        assert_eq!(bl3_profile.profile_data.echo_themes_unlocked(), 17);
        assert_eq!(bl3_profile.profile_data.profile_emotes_unlocked(), 17);
        assert_eq!(bl3_profile.profile_data.room_decorations_unlocked(), 26);
        assert_eq!(bl3_profile.profile_data.weapon_skins_unlocked(), 7);
        assert_eq!(bl3_profile.profile_data.weapon_trinkets_unlocked(), 8);
    }

    #[test]
    fn test_from_data_ps4_1() {
        let filename = Path::new("./test_files/2profps4.sav");

        let mut profile_file_data = fs::read(&filename).expect("failed to read test_file");

        let bl3_profile =
            Bl3Profile::from_bytes(filename, &mut profile_file_data, HeaderType::Ps4Profile)
                .expect("failed to read test profile");

        assert_eq!(bl3_profile.profile_data.golden_keys(), 69420);
        assert_eq!(bl3_profile.profile_data.diamond_keys(), 0);
        assert_eq!(bl3_profile.profile_data.vault_card_1_keys(), 0);
        assert_eq!(bl3_profile.profile_data.vault_card_1_chests(), 0);
        assert_eq!(bl3_profile.profile_data.guardian_rank(), 69420);
        assert_eq!(bl3_profile.profile_data.guardian_tokens(), 99999999);
        assert_eq!(
            bl3_profile
                .profile_data
                .borderlands_science_info()
                .science_level,
            BorderlandsScienceLevel::None
        );
        assert_eq!(
            bl3_profile.profile_data.borderlands_science_info().solves,
            0
        );
        assert_eq!(
            bl3_profile.profile_data.borderlands_science_info().tokens,
            69420
        );
        assert_eq!(
            *bl3_profile.profile_data.sdu_slots(),
            vec![
                ProfileSduSlotData {
                    sdu: ProfileSduSlot::Bank,
                    current: 23,
                    max: ProfileSduSlot::Bank.maximum(),
                },
                ProfileSduSlotData {
                    sdu: ProfileSduSlot::LostLoot,
                    current: 8,
                    max: ProfileSduSlot::LostLoot.maximum(),
                },
            ]
        );

        assert_eq!(bl3_profile.profile_data.bank_items().len(), 2000);
        assert_eq!(bl3_profile.profile_data.lost_loot_items().len(), 0);
        assert_eq!(bl3_profile.profile_data.character_skins_unlocked(), 204);
        assert_eq!(bl3_profile.profile_data.character_heads_unlocked(), 136);
        assert_eq!(bl3_profile.profile_data.echo_themes_unlocked(), 55);
        assert_eq!(bl3_profile.profile_data.profile_emotes_unlocked(), 64);
        assert_eq!(bl3_profile.profile_data.room_decorations_unlocked(), 94);
        assert_eq!(bl3_profile.profile_data.weapon_skins_unlocked(), 24);
        assert_eq!(bl3_profile.profile_data.weapon_trinkets_unlocked(), 63);
    }
}
