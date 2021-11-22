use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use byteorder::{LittleEndian, WriteBytesExt};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::bl3_save::character_data::CharacterData;
use crate::bl3_save::inventory_slot::InventorySlot;
use crate::file_helper::FileData;
use crate::models::CustomFormatData;
use crate::parser::{decrypt, encrypt, HeaderType};
use crate::{file_helper, parser};

pub mod ammo;
pub mod challenge_data;
pub mod character_data;
pub mod fast_travel_unlock_data;
pub mod inventory_slot;
pub mod level_data;
pub mod models;
pub mod player_class;
pub mod playthrough;
pub mod sdu;
pub mod util;

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bl3Save {
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
    pub character_data: CharacterData,
}

impl Bl3Save {
    pub fn from_file_data(file_data: &FileData, header_type: HeaderType) -> Result<Self> {
        let remaining_data = file_data.remaining_data;

        let character = decrypt(remaining_data, &header_type)?;

        let character_data = CharacterData::from_character(character)?;

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
            character_data,
        })
    }

    pub fn from_bytes(file_name: &Path, data: &[u8], header_type: HeaderType) -> Result<Self> {
        let file_data = file_helper::read_bytes(file_name, data)?;

        Self::from_file_data(&file_data, header_type)
    }

    pub fn as_bytes(&self) -> Result<(Vec<u8>, Bl3Save)> {
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

        let mut data = protobuf::Message::write_to_bytes(&self.character_data.character)?;

        encrypt(&mut data, self.header_type)?;

        output.write_u32::<LittleEndian>(data.len() as u32)?;
        output.append(&mut data);

        //Now try re-reading it also - there's no point making an invalid save
        let file_name = Path::new(&self.file_name);
        let new_save = Self::from_bytes(file_name, &output, self.header_type)?;

        Ok((output, new_save))
    }
}

impl std::fmt::Display for Bl3Save {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Savegame version: {}", self.save_game_version)?;
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
        writeln!(f, "Player Class: {}", self.character_data.player_class())?;
        writeln!(f, "XP: {}", self.character_data.character.experience_points)?;
        writeln!(f, "Level: {}", self.character_data.player_level())?;
        writeln!(f, "Guardian Rank: {}", self.character_data.guardian_rank())?;
        writeln!(f, "Money: {}", self.character_data.money())?;
        writeln!(f, "Eridium: {}", self.character_data.eridium())?;
        writeln!(
            f,
            "Playthroughs Completed: {}",
            self.character_data.character.playthroughs_completed
        )?;

        for (i, pt) in self.character_data.playthroughs().iter().enumerate() {
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
                .unlockable_inventory_slots()
                .par_iter()
                .find_first(|is| is.slot == i)
            {
                writeln!(f, "{:>1}- {}: {}", " ", slot.slot, slot.unlocked)?;
            }
        }

        writeln!(f, "SDUs:")?;

        for slot in self.character_data.sdu_slots() {
            writeln!(f, "{:>1}- {}: {}/{}", " ", slot.sdu, slot.current, slot.max)?;
        }

        writeln!(f, "Ammo Pools:")?;

        for ammo in self.character_data.ammo_pools() {
            writeln!(f, "{:>1}- {}: {}", " ", ammo.pool, ammo.current)?;
        }

        writeln!(f, "Challenge Milestones:")?;

        for challenge in self.character_data.challenge_milestones() {
            writeln!(
                f,
                "{:>1}- {}: {}",
                " ", challenge.challenge, challenge.unlocked
            )?;
        }

        writeln!(f, "Unlocked Vehicle Parts:")?;

        for v_data in self.character_data.vehicle_data() {
            writeln!(
                f,
                "{:>1}- {} {}: {}/{}",
                " ",
                v_data.vehicle_type,
                v_data.vehicle_type.subtype_name(),
                v_data.current,
                v_data.vehicle_type.maximum(),
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::bl3_save::ammo::{AmmoPool, AmmoPoolData};
    use crate::bl3_save::challenge_data::{Challenge, ChallengeData};
    use crate::bl3_save::inventory_slot::InventorySlotData;
    use crate::bl3_save::player_class::PlayerClass;
    use crate::bl3_save::sdu::{SaveSduSlot, SaveSduSlotData};
    use crate::vehicle_data::{VehicleData, VehicleSubType, VehicleType};

    use super::*;

    #[test]
    fn test_from_data_pc_1() {
        let filename = Path::new("./test_files/19.sav");

        let mut save_file_data = fs::read(&filename).expect("failed to read test_file");

        let bl3_save = Bl3Save::from_bytes(filename, &mut save_file_data, HeaderType::PcSave)
            .expect("failed to read test save");

        assert_eq!(
            bl3_save.character_data.character.preferred_character_name,
            "Ricochet Witch 5.0"
        );
        assert_eq!(bl3_save.character_data.character.save_game_id, 25);
        assert_eq!(
            bl3_save.character_data.character.save_game_guid,
            "65FAB86B4A165E6F6E844DA346BB75E2"
        );
        assert_eq!(bl3_save.character_data.player_class(), PlayerClass::Siren);
        assert_eq!(bl3_save.character_data.character.experience_points, 7149982);
        assert_eq!(bl3_save.character_data.player_level(), 65);
        assert_eq!(bl3_save.character_data.guardian_rank(), 226);
        assert_eq!(bl3_save.character_data.money(), 36575378);
        assert_eq!(bl3_save.character_data.eridium(), 48130);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 2);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs()
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 0);
        assert_eq!(
            first_playthrough.current_map.name,
            "Midnight's Cairn (Maliwan Takedown)"
        );
        assert_eq!(
            first_playthrough.active_missions,
            vec![
                "Capture the Frag",
                "Don't Truck with Eden-6",
                "Dynasty Diner",
                "Irregular Customers",
                "Kill Killavolt",
                "Malevolent Practice",
                "Proof of Wife",
                "Swamp Bro",
                "Technical NOGout",
            ]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 63);
        assert_eq!(first_playthrough.mission_milestones, vec!["Main Game"]);

        let second_playthrough = bl3_save
            .character_data
            .playthroughs()
            .get(1)
            .expect("failed to read second playthrough");

        assert_eq!(second_playthrough.mayhem_level, 10);
        assert_eq!(second_playthrough.mayhem_random_seed, -1367132962);
        assert_eq!(second_playthrough.current_map.name, "Sanctuary");
        assert_eq!(
            second_playthrough.active_missions,
            vec![
                "Bad Reception",
                "Bloody Harvest: The Rebloodening",
                "Cistern of Slaughter",
                "Dynasty Dash: Devil's Razor",
                "Invasion of Privacy",
                "Let's Get It Vaughn",
                "Maliwannabees",
                "One Man's Treasure",
                "Takedown at the Guardian Breach",
                "Takedown at the Maliwan Blacksite",
                "The Feeble and the Furious",
                "Witch's Brew",
            ]
        );
        assert_eq!(second_playthrough.missions_completed.len(), 115);
        assert_eq!(
            second_playthrough.mission_milestones,
            vec![
                "Main Game",
                "DLC2 - Guns, Love, and Tentacles",
                "DLC3 - Bounty of Blood",
            ]
        );

        assert_eq!(
            *bl3_save.character_data.unlockable_inventory_slots(),
            vec![
                InventorySlotData {
                    slot: InventorySlot::Weapon1,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon2,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon3,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon4,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Shield,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Grenade,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::ClassMod,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Artifact,
                    unlocked: true,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.sdu_slots(),
            vec![
                SaveSduSlotData {
                    sdu: SaveSduSlot::Backpack,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Sniper,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Shotgun,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Pistol,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Grenade,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Smg,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Ar,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Heavy,
                    current: 13,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.ammo_pools(),
            vec![
                AmmoPoolData {
                    pool: AmmoPool::Grenade,
                    current: 11,
                    max: AmmoPool::Grenade.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Pistol,
                    current: 201,
                    max: AmmoPool::Pistol.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Shotgun,
                    current: 110,
                    max: AmmoPool::Shotgun.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Smg,
                    current: 1800,
                    max: AmmoPool::Smg.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Ar,
                    current: 705,
                    max: AmmoPool::Ar.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Sniper,
                    current: 41,
                    max: AmmoPool::Sniper.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Heavy,
                    current: 51,
                    max: AmmoPool::Heavy.maximum(),
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.challenge_milestones(),
            vec![
                ChallengeData {
                    challenge: Challenge::ArtifactSlot,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::EridianAnalyzer,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::EridianResonator,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::MayhemMode,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::SirenClassModSlot,
                    unlocked: true,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.vehicle_data(),
            [
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Chassis),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Parts),
                    current: 12,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Skins),
                    current: 20,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Chassis),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Parts),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Skins),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Chassis),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Parts),
                    current: 12,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Skins),
                    current: 20,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Chassis),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Parts),
                    current: 9,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Skins),
                    current: 18,
                },
            ]
        );
    }

    #[test]
    fn test_from_data_pc_2() {
        let filename = Path::new("./test_files/1.sav");

        let mut save_file_data = fs::read(&filename).expect("failed to read mut test_file");

        let bl3_save = Bl3Save::from_bytes(filename, &mut save_file_data, HeaderType::PcSave)
            .expect("failed to read test save");

        assert_eq!(
            bl3_save.character_data.character.preferred_character_name,
            "Amara"
        );
        assert_eq!(bl3_save.character_data.character.save_game_id, 1);
        assert_eq!(
            bl3_save.character_data.character.save_game_guid,
            "2DF92B8B41E9A19BF98AEAB6E176B094"
        );
        assert_eq!(bl3_save.character_data.player_class(), PlayerClass::Siren);
        assert_eq!(bl3_save.character_data.character.experience_points, 1253222);
        assert_eq!(bl3_save.character_data.player_level(), 34);
        assert_eq!(bl3_save.character_data.guardian_rank(), 200);
        assert_eq!(bl3_save.character_data.money(), 89828);
        assert_eq!(bl3_save.character_data.eridium(), 367);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 1);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs()
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 1);
        assert_eq!(first_playthrough.mayhem_random_seed, 0);
        assert_eq!(first_playthrough.current_map.name, "Sanctuary");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Golden Calves", "Kill Killavolt", "Technical NOGout"]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 29);
        assert_eq!(first_playthrough.mission_milestones, vec!["Main Game"]);

        assert_eq!(
            *bl3_save.character_data.unlockable_inventory_slots(),
            vec![
                InventorySlotData {
                    slot: InventorySlot::Weapon1,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon2,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon3,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon4,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Shield,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Grenade,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::ClassMod,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Artifact,
                    unlocked: true,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.sdu_slots(),
            vec![
                SaveSduSlotData {
                    sdu: SaveSduSlot::Backpack,
                    current: 0,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Sniper,
                    current: 3,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Shotgun,
                    current: 4,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Pistol,
                    current: 3,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Grenade,
                    current: 2,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Smg,
                    current: 4,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Ar,
                    current: 3,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Heavy,
                    current: 3,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.ammo_pools(),
            vec![
                AmmoPoolData {
                    pool: AmmoPool::Grenade,
                    current: 5,
                    max: AmmoPool::Grenade.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Pistol,
                    current: 238,
                    max: AmmoPool::Pistol.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Shotgun,
                    current: 160,
                    max: AmmoPool::Shotgun.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Smg,
                    current: 1080,
                    max: AmmoPool::Smg.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Ar,
                    current: 672,
                    max: AmmoPool::Ar.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Sniper,
                    current: 84,
                    max: AmmoPool::Sniper.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Heavy,
                    current: 21,
                    max: AmmoPool::Heavy.maximum(),
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.challenge_milestones(),
            vec![
                ChallengeData {
                    challenge: Challenge::ArtifactSlot,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::EridianAnalyzer,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::EridianResonator,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::MayhemMode,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::SirenClassModSlot,
                    unlocked: false,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.vehicle_data(),
            [
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Chassis),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Parts),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Skins),
                    current: 8,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Chassis),
                    current: 2,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Parts),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Skins),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Chassis),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Parts),
                    current: 6,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Skins),
                    current: 12,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Chassis),
                    current: 2,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Parts),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Skins),
                    current: 10,
                },
            ]
        );
    }

    #[test]
    fn test_from_data_pc_3() {
        let filename = Path::new("./test_files/5.sav");

        let mut save_file_data = fs::read(&filename).expect("failed to read mut test_file");

        let bl3_save = Bl3Save::from_bytes(filename, &mut save_file_data, HeaderType::PcSave)
            .expect("failed to read test save");

        assert_eq!(
            bl3_save.character_data.character.preferred_character_name,
            "FL4K"
        );
        assert_eq!(bl3_save.character_data.character.save_game_id, 5);
        assert_eq!(
            bl3_save.character_data.character.save_game_guid,
            "4874AB774067B699E964DCAD69E5272E"
        );
        assert_eq!(
            bl3_save.character_data.player_class(),
            PlayerClass::BeastMaster
        );
        assert_eq!(bl3_save.character_data.character.experience_points, 305);
        assert_eq!(bl3_save.character_data.player_level(), 1);
        assert_eq!(bl3_save.character_data.guardian_rank(), 200);
        assert_eq!(bl3_save.character_data.money(), 35);
        assert_eq!(bl3_save.character_data.eridium(), 0);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 0);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs()
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 9573);
        assert_eq!(first_playthrough.current_map.name, "Covenant Pass");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Children of the Vault"]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 0);
        assert_eq!(first_playthrough.mission_milestones.len(), 0);

        assert_eq!(
            *bl3_save.character_data.unlockable_inventory_slots(),
            vec![
                InventorySlotData {
                    slot: InventorySlot::Weapon1,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon2,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon3,
                    unlocked: false,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon4,
                    unlocked: false,
                },
                InventorySlotData {
                    slot: InventorySlot::Shield,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Grenade,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::ClassMod,
                    unlocked: false,
                },
                InventorySlotData {
                    slot: InventorySlot::Artifact,
                    unlocked: false,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.sdu_slots(),
            vec![
                SaveSduSlotData {
                    sdu: SaveSduSlot::Backpack,
                    current: 0,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Sniper,
                    current: 0,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Shotgun,
                    current: 0,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Pistol,
                    current: 0,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Grenade,
                    current: 0,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Smg,
                    current: 0,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Ar,
                    current: 0,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Heavy,
                    current: 0,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.ammo_pools(),
            vec![
                AmmoPoolData {
                    pool: AmmoPool::Grenade,
                    current: 0,
                    max: AmmoPool::Grenade.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Pistol,
                    current: 173,
                    max: AmmoPool::Pistol.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Shotgun,
                    current: 64,
                    max: AmmoPool::Shotgun.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Smg,
                    current: 138,
                    max: AmmoPool::Smg.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Ar,
                    current: 280,
                    max: AmmoPool::Ar.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Sniper,
                    current: 27,
                    max: AmmoPool::Sniper.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Heavy,
                    current: 0,
                    max: AmmoPool::Heavy.maximum(),
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.challenge_milestones(),
            vec![
                ChallengeData {
                    challenge: Challenge::ArtifactSlot,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::EridianAnalyzer,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::EridianResonator,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::MayhemMode,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::BeastMasterClassModSlot,
                    unlocked: false,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.vehicle_data(),
            [
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Chassis),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Parts),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Skins),
                    current: 7,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Chassis),
                    current: 2,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Parts),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Skins),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Parts),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Skins),
                    current: 6,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Parts),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Skins),
                    current: 8,
                },
            ]
        );
    }

    #[test]
    fn test_from_data_pc_4() {
        let filename = Path::new("./test_files/310pc.sav");

        let mut save_file_data = fs::read(&filename).expect("failed to read mut test_file");

        let bl3_save = Bl3Save::from_bytes(filename, &mut save_file_data, HeaderType::PcSave)
            .expect("failed to read test save");

        assert_eq!(
            bl3_save.character_data.character.preferred_character_name,
            "Victory rush 5"
        );
        assert_eq!(bl3_save.character_data.character.save_game_id, 784);
        assert_eq!(
            bl3_save.character_data.character.save_game_guid,
            "E1D65E314B7FC8009A31B483A6B83F21"
        );
        assert_eq!(
            bl3_save.character_data.player_class(),
            PlayerClass::Operative
        );
        assert_eq!(bl3_save.character_data.character.experience_points, 5714393);
        assert_eq!(bl3_save.character_data.player_level(), 60);
        assert_eq!(bl3_save.character_data.guardian_rank(), 0);
        assert_eq!(bl3_save.character_data.money(), 100000000);
        assert_eq!(bl3_save.character_data.eridium(), 0);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 0);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs()
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 20046);
        assert_eq!(first_playthrough.current_map.name, "Sanctuary");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Hostile Takeover", "The Handsome Jackpot"]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 5);
        assert_eq!(first_playthrough.mission_milestones.len(), 0);

        assert_eq!(
            *bl3_save.character_data.unlockable_inventory_slots(),
            vec![
                InventorySlotData {
                    slot: InventorySlot::Weapon1,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon2,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon3,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon4,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Shield,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Grenade,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::ClassMod,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Artifact,
                    unlocked: true,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.sdu_slots(),
            vec![
                SaveSduSlotData {
                    sdu: SaveSduSlot::Backpack,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Sniper,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Shotgun,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Pistol,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Grenade,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Smg,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Ar,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Heavy,
                    current: 13,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.ammo_pools(),
            vec![
                AmmoPoolData {
                    pool: AmmoPool::Grenade,
                    current: 3,
                    max: AmmoPool::Grenade.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Pistol,
                    current: 145,
                    max: AmmoPool::Pistol.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Shotgun,
                    current: 80,
                    max: AmmoPool::Shotgun.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Smg,
                    current: 360,
                    max: AmmoPool::Smg.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Ar,
                    current: 280,
                    max: AmmoPool::Ar.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Sniper,
                    current: 48,
                    max: AmmoPool::Sniper.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Heavy,
                    current: 0,
                    max: AmmoPool::Heavy.maximum(),
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.challenge_milestones(),
            vec![
                ChallengeData {
                    challenge: Challenge::ArtifactSlot,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::EridianAnalyzer,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::EridianResonator,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::MayhemMode,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::OperativeClassModSlot,
                    unlocked: false,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.vehicle_data(),
            [
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Chassis),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Parts),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Skins),
                    current: 7,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Chassis),
                    current: 2,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Parts),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Skins),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Parts),
                    current: 5,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Skins),
                    current: 7,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Parts),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Skins),
                    current: 8,
                },
            ]
        );
    }

    #[test]
    fn test_from_data_pc_5() {
        let filename = Path::new("./test_files/quick.sav");

        let mut save_file_data = fs::read(&filename).expect("failed to read mut test_file");

        let bl3_save = Bl3Save::from_bytes(filename, &mut save_file_data, HeaderType::PcSave)
            .expect("failed to read test save");

        assert_eq!(
            bl3_save.character_data.character.preferred_character_name,
            "FL4K"
        );
        assert_eq!(bl3_save.character_data.character.save_game_id, 2);
        assert_eq!(
            bl3_save.character_data.character.save_game_guid,
            "DBA7CF0B4B0997E60920419B4E0C7D7A"
        );
        assert_eq!(
            bl3_save.character_data.player_class(),
            PlayerClass::BeastMaster
        );
        assert_eq!(bl3_save.character_data.character.experience_points, 3429728);
        assert_eq!(bl3_save.character_data.player_level(), 50);
        assert_eq!(bl3_save.character_data.guardian_rank(), 0);
        assert_eq!(bl3_save.character_data.money(), 733664);
        assert_eq!(bl3_save.character_data.eridium(), 10046);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 0);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs()
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 0);
        assert_eq!(first_playthrough.current_map.name, "Desolation's Edge");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Footsteps of Giants"]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 20);
        assert_eq!(first_playthrough.mission_milestones.len(), 0);

        assert_eq!(
            *bl3_save.character_data.unlockable_inventory_slots(),
            vec![
                InventorySlotData {
                    slot: InventorySlot::Weapon1,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon2,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon3,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon4,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Shield,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Grenade,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::ClassMod,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Artifact,
                    unlocked: true,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.sdu_slots(),
            vec![
                SaveSduSlotData {
                    sdu: SaveSduSlot::Backpack,
                    current: 8,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Sniper,
                    current: 8,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Shotgun,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Pistol,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Grenade,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Smg,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Ar,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Heavy,
                    current: 8,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.ammo_pools(),
            vec![
                AmmoPoolData {
                    pool: AmmoPool::Grenade,
                    current: 11,
                    max: AmmoPool::Grenade.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Pistol,
                    current: 1000,
                    max: AmmoPool::Pistol.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Shotgun,
                    current: 240,
                    max: AmmoPool::Shotgun.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Smg,
                    current: 1368,
                    max: AmmoPool::Smg.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Ar,
                    current: 1400,
                    max: AmmoPool::Ar.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Sniper,
                    current: 144,
                    max: AmmoPool::Sniper.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Heavy,
                    current: 36,
                    max: AmmoPool::Heavy.maximum(),
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.challenge_milestones(),
            vec![
                ChallengeData {
                    challenge: Challenge::ArtifactSlot,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::EridianAnalyzer,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::EridianResonator,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::MayhemMode,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::BeastMasterClassModSlot,
                    unlocked: true,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.vehicle_data(),
            [
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Chassis),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Parts),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Skins),
                    current: 8,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Parts),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Skins),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Chassis),
                    current: 2,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Parts),
                    current: 5,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Skins),
                    current: 12,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Chassis),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Parts),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Skins),
                    current: 9,
                },
            ]
        );
    }

    #[test]
    fn test_from_data_ps4_1() {
        let filename = Path::new("./test_files/1ps4_v100.sav");

        let mut save_file_data = fs::read(&filename).expect("failed to read test_file");

        let bl3_save = Bl3Save::from_bytes(filename, &mut save_file_data, HeaderType::Ps4Save)
            .expect("failed to read test save");

        assert_eq!(
            bl3_save.character_data.character.preferred_character_name,
            "Amara"
        );
        assert_eq!(bl3_save.character_data.character.save_game_id, 1);
        assert_eq!(bl3_save.character_data.character.save_game_guid, "");
        assert_eq!(bl3_save.character_data.player_class(), PlayerClass::Siren);
        assert_eq!(bl3_save.character_data.character.experience_points, 0);
        assert_eq!(bl3_save.character_data.player_level(), 1);
        assert_eq!(bl3_save.character_data.guardian_rank(), 0);
        assert_eq!(bl3_save.character_data.money(), 0);
        assert_eq!(bl3_save.character_data.eridium(), 0);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 0);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs()
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 0);
        assert_eq!(first_playthrough.current_map.name, "Covenant Pass");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Children of the Vault"]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 0);
        assert_eq!(first_playthrough.mission_milestones.len(), 0);

        assert_eq!(
            *bl3_save.character_data.unlockable_inventory_slots(),
            vec![
                InventorySlotData {
                    slot: InventorySlot::Weapon1,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon2,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon3,
                    unlocked: false,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon4,
                    unlocked: false,
                },
                InventorySlotData {
                    slot: InventorySlot::Shield,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Grenade,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::ClassMod,
                    unlocked: false,
                },
                InventorySlotData {
                    slot: InventorySlot::Artifact,
                    unlocked: false,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.sdu_slots(),
            vec![
                SaveSduSlotData {
                    sdu: SaveSduSlot::Backpack,
                    current: 0,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Sniper,
                    current: 0,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Shotgun,
                    current: 0,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Pistol,
                    current: 0,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Grenade,
                    current: 0,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Smg,
                    current: 0,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Ar,
                    current: 0,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Heavy,
                    current: 0,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.ammo_pools(),
            vec![
                AmmoPoolData {
                    pool: AmmoPool::Grenade,
                    current: 0,
                    max: AmmoPool::Grenade.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Pistol,
                    current: 48,
                    max: AmmoPool::Pistol.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Shotgun,
                    current: 0,
                    max: AmmoPool::Shotgun.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Smg,
                    current: 0,
                    max: AmmoPool::Smg.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Ar,
                    current: 0,
                    max: AmmoPool::Ar.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Sniper,
                    current: 0,
                    max: AmmoPool::Sniper.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Heavy,
                    current: 0,
                    max: AmmoPool::Heavy.maximum(),
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.challenge_milestones(),
            vec![
                ChallengeData {
                    challenge: Challenge::ArtifactSlot,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::EridianAnalyzer,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::EridianResonator,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::MayhemMode,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::SirenClassModSlot,
                    unlocked: false,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.vehicle_data(),
            [
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Chassis),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Parts),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Skins),
                    current: 7,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Parts),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Skins),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Parts),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Skins),
                    current: 6,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Parts),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Skins),
                    current: 8,
                },
            ]
        );
    }

    #[test]
    fn test_from_data_ps4_2() {
        let filename = Path::new("./test_files/69ps4_v103.sav");

        let mut save_file_data = fs::read(&filename).expect("failed to read test_file");

        let bl3_save = Bl3Save::from_bytes(filename, &mut save_file_data, HeaderType::Ps4Save)
            .expect("failed to read test save");

        assert_eq!(
            bl3_save.character_data.character.preferred_character_name,
            "Amara"
        );
        assert_eq!(bl3_save.character_data.character.save_game_id, 1);
        assert_eq!(
            bl3_save.character_data.character.save_game_guid,
            "072F929508D737DDFEB3AE9A060A23D7"
        );
        assert_eq!(bl3_save.character_data.player_class(), PlayerClass::Siren);
        assert_eq!(bl3_save.character_data.character.experience_points, 7149982);
        assert_eq!(bl3_save.character_data.player_level(), 65);
        assert_eq!(bl3_save.character_data.guardian_rank(), 0);
        assert_eq!(bl3_save.character_data.money(), 999999999);
        assert_eq!(bl3_save.character_data.eridium(), 1000000879);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 1);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs()
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 0);
        assert_eq!(first_playthrough.current_map.name, "Sanctuary");
        assert_eq!(first_playthrough.active_missions, vec!["Fire in the Sky"]);
        assert_eq!(first_playthrough.missions_completed.len(), 86);
        assert_eq!(first_playthrough.mission_milestones, vec!["Main Game"]);

        assert_eq!(
            *bl3_save.character_data.unlockable_inventory_slots(),
            vec![
                InventorySlotData {
                    slot: InventorySlot::Weapon1,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon2,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon3,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon4,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Shield,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Grenade,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::ClassMod,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Artifact,
                    unlocked: true,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.sdu_slots(),
            vec![
                SaveSduSlotData {
                    sdu: SaveSduSlot::Backpack,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Sniper,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Shotgun,
                    current: 10,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Pistol,
                    current: 10,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Grenade,
                    current: 10,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Smg,
                    current: 10,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Ar,
                    current: 10,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Heavy,
                    current: 13,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.ammo_pools(),
            vec![
                AmmoPoolData {
                    pool: AmmoPool::Grenade,
                    current: 11,
                    max: AmmoPool::Grenade.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Pistol,
                    current: 1000,
                    max: AmmoPool::Pistol.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Shotgun,
                    current: 240,
                    max: AmmoPool::Shotgun.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Smg,
                    current: 1800,
                    max: AmmoPool::Smg.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Ar,
                    current: 662,
                    max: AmmoPool::Ar.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Sniper,
                    current: 144,
                    max: AmmoPool::Sniper.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Heavy,
                    current: 36,
                    max: AmmoPool::Heavy.maximum(),
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.challenge_milestones(),
            vec![
                ChallengeData {
                    challenge: Challenge::ArtifactSlot,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::EridianAnalyzer,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::EridianResonator,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::MayhemMode,
                    unlocked: true,
                },
                ChallengeData {
                    challenge: Challenge::SirenClassModSlot,
                    unlocked: false,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.vehicle_data(),
            [
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Chassis),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Parts),
                    current: 8,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Skins),
                    current: 13,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Parts),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Skins),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Chassis),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Parts),
                    current: 7,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Skins),
                    current: 16,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Chassis),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Parts),
                    current: 5,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Skins),
                    current: 12,
                },
            ]
        );
    }

    #[test]
    fn test_from_data_ps4_3() {
        let filename = Path::new("./test_files/310ps4.sav");

        let mut save_file_data = fs::read(&filename).expect("failed to read test_file");

        let bl3_save = Bl3Save::from_bytes(filename, &mut save_file_data, HeaderType::Ps4Save)
            .expect("failed to read test save");

        assert_eq!(
            bl3_save.character_data.character.preferred_character_name,
            "Victory rush 5"
        );
        assert_eq!(bl3_save.character_data.character.save_game_id, 784);
        assert_eq!(
            bl3_save.character_data.character.save_game_guid,
            "E1D65E314B7FC8009A31B483A6B83F21"
        );
        assert_eq!(
            bl3_save.character_data.player_class(),
            PlayerClass::Operative
        );
        assert_eq!(bl3_save.character_data.character.experience_points, 5714393);
        assert_eq!(bl3_save.character_data.player_level(), 60);
        assert_eq!(bl3_save.character_data.guardian_rank(), 0);
        assert_eq!(bl3_save.character_data.money(), 100000000);
        assert_eq!(bl3_save.character_data.eridium(), 0);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 0);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs()
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 20046);
        assert_eq!(first_playthrough.current_map.name, "Sanctuary");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Hostile Takeover", "The Handsome Jackpot"]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 5);
        assert_eq!(first_playthrough.mission_milestones.len(), 0);

        assert_eq!(
            *bl3_save.character_data.unlockable_inventory_slots(),
            vec![
                InventorySlotData {
                    slot: InventorySlot::Weapon1,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon2,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon3,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Weapon4,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Shield,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Grenade,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::ClassMod,
                    unlocked: true,
                },
                InventorySlotData {
                    slot: InventorySlot::Artifact,
                    unlocked: true,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.sdu_slots(),
            vec![
                SaveSduSlotData {
                    sdu: SaveSduSlot::Backpack,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Sniper,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Shotgun,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Pistol,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Grenade,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Smg,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Ar,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    sdu: SaveSduSlot::Heavy,
                    current: 13,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.ammo_pools(),
            vec![
                AmmoPoolData {
                    pool: AmmoPool::Grenade,
                    current: 3,
                    max: AmmoPool::Grenade.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Pistol,
                    current: 145,
                    max: AmmoPool::Pistol.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Shotgun,
                    current: 80,
                    max: AmmoPool::Shotgun.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Smg,
                    current: 360,
                    max: AmmoPool::Smg.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Ar,
                    current: 280,
                    max: AmmoPool::Ar.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Sniper,
                    current: 48,
                    max: AmmoPool::Sniper.maximum(),
                },
                AmmoPoolData {
                    pool: AmmoPool::Heavy,
                    current: 0,
                    max: AmmoPool::Heavy.maximum(),
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.challenge_milestones(),
            vec![
                ChallengeData {
                    challenge: Challenge::ArtifactSlot,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::EridianAnalyzer,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::EridianResonator,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::MayhemMode,
                    unlocked: false,
                },
                ChallengeData {
                    challenge: Challenge::OperativeClassModSlot,
                    unlocked: false,
                },
            ]
        );

        assert_eq!(
            *bl3_save.character_data.vehicle_data(),
            [
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Chassis),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Parts),
                    current: 4,
                },
                VehicleData {
                    vehicle_type: VehicleType::Outrunner(VehicleSubType::Skins),
                    current: 7,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Chassis),
                    current: 2,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Parts),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Jetbeast(VehicleSubType::Skins),
                    current: 1,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Parts),
                    current: 5,
                },
                VehicleData {
                    vehicle_type: VehicleType::Technical(VehicleSubType::Skins),
                    current: 7,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Chassis),
                    current: 0,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Parts),
                    current: 3,
                },
                VehicleData {
                    vehicle_type: VehicleType::Cyclone(VehicleSubType::Skins),
                    current: 8,
                },
            ]
        );
    }
}
