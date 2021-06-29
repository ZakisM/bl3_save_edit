use std::fmt;
use std::fmt::Formatter;

use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::bl3_save::character_data::CharacterData;
use crate::bl3_save::inventory_slot::InventorySlot;
use crate::file_helper;
use crate::file_helper::FileData;
use crate::models::CustomFormatData;
use crate::parser::{decrypt, FileType};

mod ammo;
mod character_data;
mod inventory_slot;
mod models;
mod player_class;
mod sdu;
mod util;

#[derive(Debug)]
pub struct Bl3Save {
    save_game_version: u32,
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
    character_data: CharacterData,
}

impl Bl3Save {
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

        let character = decrypt(remaining_data, file_type)?;

        let character_data = CharacterData::from_character(character)?;

        Ok(Self {
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
            character_data,
        })
    }
}

impl fmt::Display for Bl3Save {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // writeln!(f, "Savegame version: {}", self.save_game_version)?;
        // writeln!(f, "Package version: {}", self.package_version)?;
        // writeln!(
        //     f,
        //     "Engine version: {}.{}.{}.{}",
        //     self.engine_major, self.engine_minor, self.engine_patch, self.engine_build
        // )?;
        // writeln!(f, "Build ID: {}", self.build_id)?;
        // writeln!(f, "Custom Format Version: {}", self.custom_format_version)?;
        // writeln!(
        //     f,
        //     "Custom Format Data Count: {}",
        //     self.custom_format_data_count
        // )?;
        // writeln!(f, "Savegame type: {}", self.save_game_type)
        writeln!(
            f,
            "Character: {}",
            self.character_data.character.preferred_character_name
        )?;
        writeln!(
            f,
            "Savegame ID: {}",
            self.character_data.character.save_game_id
        )?;
        writeln!(
            f,
            "Savegame GUID: {}",
            self.character_data.character.save_game_guid
        )?;
        writeln!(f, "Player Class: {}", self.character_data.player_class)?;
        writeln!(f, "XP: {}", self.character_data.character.experience_points)?;
        writeln!(f, "Level: {}", self.character_data.player_level)?;
        writeln!(f, "Guardian Rank: {}", self.character_data.guardian_rank)?;
        writeln!(f, "Money: {}", self.character_data.money)?;
        writeln!(f, "Eridium: {}", self.character_data.eridium)?;
        writeln!(
            f,
            "Playthroughs Completed: {}",
            self.character_data.character.playthroughs_completed
        )?;

        for (i, pt) in self.character_data.playthroughs.iter().enumerate() {
            writeln!(f, "Playthrough {} Info:", i + 1)?;
            writeln!(f, "{:>1}- Mayhem Level: {}", " ", pt.mayhem_level)?;
            writeln!(
                f,
                "{:>1}- Mayhem Random Seed: {}",
                " ", pt.mayhem_random_seed
            )?;
            writeln!(f, "{:>1}- In Map: {}", " ", pt.current_map)?;

            if !pt.active_missions.is_empty() {
                writeln!(f, "{:>1}- Active Missions", " ")?;
                for mission in &pt.active_missions {
                    writeln!(f, "{:>2}- {}", " ", mission)?;
                }
            }

            if !pt.missions_completed.is_empty() {
                writeln!(
                    f,
                    "{:>1}- Missions Completed: {}",
                    " ",
                    pt.missions_completed.len()
                )?;

                if !pt.mission_milestones.is_empty() {
                    writeln!(f, "{:>1}- Mission Milestones:", " ")?;

                    for milestone in &pt.mission_milestones {
                        writeln!(f, "{:>2}- Finished: {}", " ", milestone)?;
                    }
                }
            }
        }

        writeln!(f, "Inventory Slots Unlocked:")?;

        for i in [
            InventorySlot::Weapon3,
            InventorySlot::Weapon4,
            InventorySlot::ClassMod,
            InventorySlot::Artifact,
        ] {
            if let Some(slot) = self
                .character_data
                .unlockable_inventory_slots
                .par_iter()
                .find_first(|is| is.slot == i)
            {
                writeln!(f, "{:>1}- {}: {}", " ", slot.slot, slot.unlocked)?;
            }
        }

        writeln!(f, "SDUs:")?;

        for slot in &self.character_data.sdu_slots {
            writeln!(
                f,
                "{:>1}- {}: {}/{}",
                " ", slot.slot, slot.current, slot.max
            )?;
        }

        writeln!(f, "Ammo Pools:")?;

        for ammo in &self.character_data.ammo_pools {
            writeln!(f, "{:>1}- {}: {}", " ", ammo.ammo, ammo.current)?;
        }

        writeln!(f, "Challenge Milestones:")?;

        for challenge in &self.character_data.challenge_milestones {
            writeln!(
                f,
                "{:>1}- {}: {}",
                " ", challenge.challenge, challenge.unlocked
            )?;
        }

        writeln!(f, "Unlocked Vehicle Parts:")?;

        for v_stat in &self.character_data.vehicle_stats {
            writeln!(
                f,
                "{:>1}- {} - Chassis (wheels): {}/{}, Parts: {}/{}, Skins: {}/{}",
                " ",
                v_stat.name,
                v_stat.chassis_count,
                v_stat.total_chassis_count,
                v_stat.parts_count,
                v_stat.total_parts_count,
                v_stat.skins_count,
                v_stat.total_skins_count,
            )?;
        }

        Ok(())
    }
}
