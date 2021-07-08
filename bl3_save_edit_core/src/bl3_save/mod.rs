use std::fmt;
use std::fmt::Formatter;

use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::bl3_save::character_data::CharacterData;
use crate::bl3_save::inventory_slot::InventorySlot;
use crate::file_helper;
use crate::file_helper::FileData;
use crate::models::CustomFormatData;
use crate::parser::{decrypt, HeaderType};

pub mod ammo;
pub mod challenge_data;
pub mod character_data;
pub mod inventory_slot;
pub mod models;
pub mod player_class;
pub mod sdu;
pub mod util;

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

//TODO: Make a method that doesn't need to calculate all character_data, only level

impl Bl3Save {
    pub fn from_file_data(file_data: &FileData, header_type: HeaderType) -> Result<Self> {
        let remaining_data = file_data.remaining_data;

        let character = decrypt(remaining_data, header_type)?;

        let character_data = CharacterData::from_character(character)?;

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
            ..
        } = file_data.clone();

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

    pub fn from_bytes(data: &[u8], header_type: HeaderType) -> Result<Self> {
        let file_data = file_helper::read_bytes(&data)?;

        Self::from_file_data(&file_data, header_type)
    }
}

impl fmt::Display for Bl3Save {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::bl3_save::ammo::{Ammo, AmmoPoolData};
    use crate::bl3_save::challenge_data::{Challenge, ChallengeData};
    use crate::bl3_save::inventory_slot::InventorySlotData;
    use crate::bl3_save::player_class::PlayerClass;
    use crate::bl3_save::sdu::{SaveSduSlot, SaveSduSlotData};
    use crate::game_data::{
        VEHICLE_CHASSIS_CYCLONE, VEHICLE_CHASSIS_JETBEAST, VEHICLE_CHASSIS_OUTRUNNER,
        VEHICLE_CHASSIS_TECHNICAL, VEHICLE_PARTS_CYCLONE, VEHICLE_PARTS_JETBEAST,
        VEHICLE_PARTS_OUTRUNNER, VEHICLE_PARTS_TECHNICAL, VEHICLE_SKINS_CYCLONE,
        VEHICLE_SKINS_JETBEAST, VEHICLE_SKINS_OUTRUNNER, VEHICLE_SKINS_TECHNICAL,
    };
    use crate::vehicle_data::{VehicleName, VehicleStats};

    use super::*;

    #[test]
    fn test_from_data_pc_1() {
        let mut save_file_data = fs::read("./test_files/19.sav").expect("failed to read test_file");
        let bl3_save = Bl3Save::from_bytes(&mut save_file_data, HeaderType::PcSave)
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
        assert_eq!(bl3_save.character_data.player_class, PlayerClass::Siren);
        assert_eq!(bl3_save.character_data.character.experience_points, 7149982);
        assert_eq!(bl3_save.character_data.player_level, 65);
        assert_eq!(bl3_save.character_data.guardian_rank, 226);
        assert_eq!(bl3_save.character_data.money, 36575378);
        assert_eq!(bl3_save.character_data.eridium, 48130);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 2);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 0);
        assert_eq!(
            first_playthrough.current_map,
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
                "Technical NOGout"
            ]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 63);
        assert_eq!(first_playthrough.mission_milestones, vec!["Main Game"]);

        let second_playthrough = bl3_save
            .character_data
            .playthroughs
            .get(1)
            .expect("failed to read second playthrough");

        assert_eq!(second_playthrough.mayhem_level, 10);
        assert_eq!(second_playthrough.mayhem_random_seed, -1367132962);
        assert_eq!(second_playthrough.current_map, "Sanctuary");
        assert_eq!(
            second_playthrough.active_missions,
            vec![
                "Bad Reception",
                "Bloody Harvest: The Rebloodening",
                "Cistern of Slaughter",
                "Dynasty Dash: Pandora",
                "Invasion of Privacy",
                "Let's Get It Vaughn",
                "Maliwannabees",
                "One Man's Treasure",
                "Takedown at the Guardian Breach",
                "Takedown at the Maliwan Blacksite",
                "The Feeble and the Furious",
                "Witch's Brew"
            ]
        );
        assert_eq!(second_playthrough.missions_completed.len(), 115);
        assert_eq!(
            second_playthrough.mission_milestones,
            vec![
                "Main Game",
                "DLC2 - Guns, Love, and Tentacles",
                "DLC3 - Bounty of Blood"
            ]
        );

        assert_eq!(
            bl3_save.character_data.unlockable_inventory_slots,
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
            bl3_save.character_data.sdu_slots,
            vec![
                SaveSduSlotData {
                    slot: SaveSduSlot::Backpack,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Sniper,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Shotgun,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Pistol,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Grenade,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Smg,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Ar,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Heavy,
                    current: 13,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.ammo_pools,
            vec![
                AmmoPoolData {
                    ammo: Ammo::Grenade,
                    current: 11,
                },
                AmmoPoolData {
                    ammo: Ammo::Pistol,
                    current: 201,
                },
                AmmoPoolData {
                    ammo: Ammo::Shotgun,
                    current: 110,
                },
                AmmoPoolData {
                    ammo: Ammo::Smg,
                    current: 1800,
                },
                AmmoPoolData {
                    ammo: Ammo::Ar,
                    current: 705,
                },
                AmmoPoolData {
                    ammo: Ammo::Sniper,
                    current: 41,
                },
                AmmoPoolData {
                    ammo: Ammo::Heavy,
                    current: 51,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.challenge_milestones,
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
            bl3_save.character_data.vehicle_stats,
            vec![
                VehicleStats {
                    name: VehicleName::Outrunner,
                    chassis_count: 4,
                    total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                    parts_count: 12,
                    total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                    skins_count: 20,
                    total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
                },
                VehicleStats {
                    name: VehicleName::Jetbeast,
                    chassis_count: 4,
                    total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                    parts_count: 4,
                    total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                    skins_count: 3,
                    total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
                },
                VehicleStats {
                    name: VehicleName::Technical,
                    chassis_count: 4,
                    total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                    parts_count: 12,
                    total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                    skins_count: 20,
                    total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
                },
                VehicleStats {
                    name: VehicleName::Cyclone,
                    chassis_count: 4,
                    total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                    parts_count: 9,
                    total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                    skins_count: 18,
                    total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
                },
            ]
        );
    }

    #[test]
    fn test_from_data_pc_2() {
        let mut save_file_data =
            fs::read("./test_files/1.sav").expect("failed to read mut test_file");
        let bl3_save = Bl3Save::from_bytes(&mut save_file_data, HeaderType::PcSave)
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
        assert_eq!(bl3_save.character_data.player_class, PlayerClass::Siren);
        assert_eq!(bl3_save.character_data.character.experience_points, 1253222);
        assert_eq!(bl3_save.character_data.player_level, 34);
        assert_eq!(bl3_save.character_data.guardian_rank, 200);
        assert_eq!(bl3_save.character_data.money, 89828);
        assert_eq!(bl3_save.character_data.eridium, 367);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 1);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 1);
        assert_eq!(first_playthrough.mayhem_random_seed, 0);
        assert_eq!(first_playthrough.current_map, "Sanctuary");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Golden Calves", "Kill Killavolt", "Technical NOGout",]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 29);
        assert_eq!(first_playthrough.mission_milestones, vec!["Main Game"]);

        assert_eq!(
            bl3_save.character_data.unlockable_inventory_slots,
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
            bl3_save.character_data.sdu_slots,
            vec![
                SaveSduSlotData {
                    slot: SaveSduSlot::Sniper,
                    current: 3,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Shotgun,
                    current: 4,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Pistol,
                    current: 3,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Grenade,
                    current: 2,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Smg,
                    current: 4,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Ar,
                    current: 3,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Heavy,
                    current: 3,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.ammo_pools,
            vec![
                AmmoPoolData {
                    ammo: Ammo::Grenade,
                    current: 5,
                },
                AmmoPoolData {
                    ammo: Ammo::Pistol,
                    current: 238,
                },
                AmmoPoolData {
                    ammo: Ammo::Shotgun,
                    current: 160,
                },
                AmmoPoolData {
                    ammo: Ammo::Smg,
                    current: 1080,
                },
                AmmoPoolData {
                    ammo: Ammo::Ar,
                    current: 672,
                },
                AmmoPoolData {
                    ammo: Ammo::Sniper,
                    current: 84,
                },
                AmmoPoolData {
                    ammo: Ammo::Heavy,
                    current: 21,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.challenge_milestones,
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
            bl3_save.character_data.vehicle_stats,
            vec![
                VehicleStats {
                    name: VehicleName::Outrunner,
                    chassis_count: 1,
                    total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                    parts_count: 4,
                    total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                    skins_count: 8,
                    total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
                },
                VehicleStats {
                    name: VehicleName::Jetbeast,
                    chassis_count: 2,
                    total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                    parts_count: 3,
                    total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                    skins_count: 1,
                    total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
                },
                VehicleStats {
                    name: VehicleName::Technical,
                    chassis_count: 3,
                    total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                    parts_count: 6,
                    total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                    skins_count: 12,
                    total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
                },
                VehicleStats {
                    name: VehicleName::Cyclone,
                    chassis_count: 2,
                    total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                    parts_count: 4,
                    total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                    skins_count: 10,
                    total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
                },
            ]
        );
    }

    #[test]
    fn test_from_data_pc_3() {
        let mut save_file_data =
            fs::read("./test_files/5.sav").expect("failed to read mut test_file");
        let bl3_save = Bl3Save::from_bytes(&mut save_file_data, HeaderType::PcSave)
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
            bl3_save.character_data.player_class,
            PlayerClass::BeastMaster
        );
        assert_eq!(bl3_save.character_data.character.experience_points, 305);
        assert_eq!(bl3_save.character_data.player_level, 1);
        assert_eq!(bl3_save.character_data.guardian_rank, 200);
        assert_eq!(bl3_save.character_data.money, 35);
        assert_eq!(bl3_save.character_data.eridium, 0);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 0);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 9573);
        assert_eq!(first_playthrough.current_map, "Covenant Pass");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Children of the Vault",]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 0);
        assert_eq!(first_playthrough.mission_milestones.len(), 0);

        assert_eq!(
            bl3_save.character_data.unlockable_inventory_slots,
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

        assert_eq!(bl3_save.character_data.sdu_slots, vec![]);

        assert_eq!(
            bl3_save.character_data.ammo_pools,
            vec![
                AmmoPoolData {
                    ammo: Ammo::Grenade,
                    current: 0,
                },
                AmmoPoolData {
                    ammo: Ammo::Pistol,
                    current: 173,
                },
                AmmoPoolData {
                    ammo: Ammo::Shotgun,
                    current: 64,
                },
                AmmoPoolData {
                    ammo: Ammo::Smg,
                    current: 138,
                },
                AmmoPoolData {
                    ammo: Ammo::Ar,
                    current: 280,
                },
                AmmoPoolData {
                    ammo: Ammo::Sniper,
                    current: 27,
                },
                AmmoPoolData {
                    ammo: Ammo::Heavy,
                    current: 0,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.challenge_milestones,
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
            bl3_save.character_data.vehicle_stats,
            vec![
                VehicleStats {
                    name: VehicleName::Outrunner,
                    chassis_count: 1,
                    total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                    parts_count: 4,
                    total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                    skins_count: 7,
                    total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
                },
                VehicleStats {
                    name: VehicleName::Jetbeast,
                    chassis_count: 2,
                    total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                    parts_count: 3,
                    total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                    skins_count: 1,
                    total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
                },
                VehicleStats {
                    name: VehicleName::Technical,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                    parts_count: 4,
                    total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                    skins_count: 6,
                    total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
                },
                VehicleStats {
                    name: VehicleName::Cyclone,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                    parts_count: 3,
                    total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                    skins_count: 8,
                    total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
                },
            ]
        );
    }

    #[test]
    fn test_from_data_pc_4() {
        let mut save_file_data =
            fs::read("./test_files/310pc.sav").expect("failed to read mut test_file");
        let bl3_save = Bl3Save::from_bytes(&mut save_file_data, HeaderType::PcSave)
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
        assert_eq!(bl3_save.character_data.player_class, PlayerClass::Operative);
        assert_eq!(bl3_save.character_data.character.experience_points, 5714393);
        assert_eq!(bl3_save.character_data.player_level, 60);
        assert_eq!(bl3_save.character_data.guardian_rank, 0);
        assert_eq!(bl3_save.character_data.money, 100000000);
        assert_eq!(bl3_save.character_data.eridium, 0);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 0);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 20046);
        assert_eq!(first_playthrough.current_map, "Sanctuary");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Hostile Takeover", "The Handsome Jackpot",]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 5);
        assert_eq!(first_playthrough.mission_milestones.len(), 0);

        assert_eq!(
            bl3_save.character_data.unlockable_inventory_slots,
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
            bl3_save.character_data.sdu_slots,
            vec![
                SaveSduSlotData {
                    slot: SaveSduSlot::Backpack,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Sniper,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Shotgun,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Pistol,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Grenade,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Smg,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Ar,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Heavy,
                    current: 13,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.ammo_pools,
            vec![
                AmmoPoolData {
                    ammo: Ammo::Grenade,
                    current: 3,
                },
                AmmoPoolData {
                    ammo: Ammo::Pistol,
                    current: 145,
                },
                AmmoPoolData {
                    ammo: Ammo::Shotgun,
                    current: 80,
                },
                AmmoPoolData {
                    ammo: Ammo::Smg,
                    current: 360,
                },
                AmmoPoolData {
                    ammo: Ammo::Ar,
                    current: 280,
                },
                AmmoPoolData {
                    ammo: Ammo::Sniper,
                    current: 48,
                },
                AmmoPoolData {
                    ammo: Ammo::Heavy,
                    current: 0,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.challenge_milestones,
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
            bl3_save.character_data.vehicle_stats,
            vec![
                VehicleStats {
                    name: VehicleName::Outrunner,
                    chassis_count: 1,
                    total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                    parts_count: 4,
                    total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                    skins_count: 7,
                    total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
                },
                VehicleStats {
                    name: VehicleName::Jetbeast,
                    chassis_count: 2,
                    total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                    parts_count: 3,
                    total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                    skins_count: 1,
                    total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
                },
                VehicleStats {
                    name: VehicleName::Technical,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                    parts_count: 5,
                    total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                    skins_count: 7,
                    total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
                },
                VehicleStats {
                    name: VehicleName::Cyclone,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                    parts_count: 3,
                    total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                    skins_count: 8,
                    total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
                },
            ]
        );
    }

    #[test]
    fn test_from_data_pc_5() {
        let mut save_file_data =
            fs::read("./test_files/quick.sav").expect("failed to read mut test_file");
        let bl3_save = Bl3Save::from_bytes(&mut save_file_data, HeaderType::PcSave)
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
            bl3_save.character_data.player_class,
            PlayerClass::BeastMaster
        );
        assert_eq!(bl3_save.character_data.character.experience_points, 3429728);
        assert_eq!(bl3_save.character_data.player_level, 50);
        assert_eq!(bl3_save.character_data.guardian_rank, 0);
        assert_eq!(bl3_save.character_data.money, 733664);
        assert_eq!(bl3_save.character_data.eridium, 10046);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 0);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 0);
        assert_eq!(first_playthrough.current_map, "Desolation's Edge");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Footsteps of Giants",]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 20);
        assert_eq!(first_playthrough.mission_milestones.len(), 0);

        assert_eq!(
            bl3_save.character_data.unlockable_inventory_slots,
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
            bl3_save.character_data.sdu_slots,
            vec![
                SaveSduSlotData {
                    slot: SaveSduSlot::Backpack,
                    current: 8,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Sniper,
                    current: 8,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Shotgun,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Pistol,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Grenade,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Smg,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Ar,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Heavy,
                    current: 8,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.ammo_pools,
            vec![
                AmmoPoolData {
                    ammo: Ammo::Grenade,
                    current: 11,
                },
                AmmoPoolData {
                    ammo: Ammo::Pistol,
                    current: 1000,
                },
                AmmoPoolData {
                    ammo: Ammo::Shotgun,
                    current: 240,
                },
                AmmoPoolData {
                    ammo: Ammo::Smg,
                    current: 1368,
                },
                AmmoPoolData {
                    ammo: Ammo::Ar,
                    current: 1400,
                },
                AmmoPoolData {
                    ammo: Ammo::Sniper,
                    current: 144,
                },
                AmmoPoolData {
                    ammo: Ammo::Heavy,
                    current: 36,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.challenge_milestones,
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
            bl3_save.character_data.vehicle_stats,
            vec![
                VehicleStats {
                    name: VehicleName::Outrunner,
                    chassis_count: 1,
                    total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                    parts_count: 4,
                    total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                    skins_count: 8,
                    total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
                },
                VehicleStats {
                    name: VehicleName::Jetbeast,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                    parts_count: 0,
                    total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                    skins_count: 0,
                    total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
                },
                VehicleStats {
                    name: VehicleName::Technical,
                    chassis_count: 2,
                    total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                    parts_count: 5,
                    total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                    skins_count: 12,
                    total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
                },
                VehicleStats {
                    name: VehicleName::Cyclone,
                    chassis_count: 1,
                    total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                    parts_count: 3,
                    total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                    skins_count: 9,
                    total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
                },
            ]
        );
    }

    #[test]
    fn test_from_data_ps4_1() {
        let mut save_file_data =
            fs::read("./test_files/1ps4_v100.sav").expect("failed to read test_file");
        let bl3_save = Bl3Save::from_bytes(&mut save_file_data, HeaderType::Ps4Save)
            .expect("failed to read test save");

        assert_eq!(
            bl3_save.character_data.character.preferred_character_name,
            "Amara"
        );
        assert_eq!(bl3_save.character_data.character.save_game_id, 1);
        assert_eq!(bl3_save.character_data.character.save_game_guid, "");
        assert_eq!(bl3_save.character_data.player_class, PlayerClass::Siren);
        assert_eq!(bl3_save.character_data.character.experience_points, 0);
        assert_eq!(bl3_save.character_data.player_level, 1);
        assert_eq!(bl3_save.character_data.guardian_rank, 0);
        assert_eq!(bl3_save.character_data.money, 0);
        assert_eq!(bl3_save.character_data.eridium, 0);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 0);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 0);
        assert_eq!(first_playthrough.current_map, "Covenant Pass");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Children of the Vault",]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 0);
        assert_eq!(first_playthrough.mission_milestones.len(), 0);

        assert_eq!(
            bl3_save.character_data.unlockable_inventory_slots,
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

        assert_eq!(bl3_save.character_data.sdu_slots.len(), 0);

        assert_eq!(
            bl3_save.character_data.ammo_pools,
            vec![
                AmmoPoolData {
                    ammo: Ammo::Grenade,
                    current: 0,
                },
                AmmoPoolData {
                    ammo: Ammo::Pistol,
                    current: 48,
                },
                AmmoPoolData {
                    ammo: Ammo::Shotgun,
                    current: 0,
                },
                AmmoPoolData {
                    ammo: Ammo::Smg,
                    current: 0,
                },
                AmmoPoolData {
                    ammo: Ammo::Ar,
                    current: 0,
                },
                AmmoPoolData {
                    ammo: Ammo::Sniper,
                    current: 0,
                },
                AmmoPoolData {
                    ammo: Ammo::Heavy,
                    current: 0,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.challenge_milestones,
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
            bl3_save.character_data.vehicle_stats,
            vec![
                VehicleStats {
                    name: VehicleName::Outrunner,
                    chassis_count: 1,
                    total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                    parts_count: 4,
                    total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                    skins_count: 7,
                    total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
                },
                VehicleStats {
                    name: VehicleName::Jetbeast,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                    parts_count: 0,
                    total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                    skins_count: 0,
                    total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
                },
                VehicleStats {
                    name: VehicleName::Technical,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                    parts_count: 4,
                    total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                    skins_count: 6,
                    total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
                },
                VehicleStats {
                    name: VehicleName::Cyclone,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                    parts_count: 3,
                    total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                    skins_count: 8,
                    total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
                },
            ]
        );
    }

    #[test]
    fn test_from_data_ps4_2() {
        let mut save_file_data =
            fs::read("./test_files/69ps4_v103.sav").expect("failed to read test_file");
        let bl3_save = Bl3Save::from_bytes(&mut save_file_data, HeaderType::Ps4Save)
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
        assert_eq!(bl3_save.character_data.player_class, PlayerClass::Siren);
        assert_eq!(bl3_save.character_data.character.experience_points, 7149982);
        assert_eq!(bl3_save.character_data.player_level, 65);
        assert_eq!(bl3_save.character_data.guardian_rank, 0);
        assert_eq!(bl3_save.character_data.money, 999999999);
        assert_eq!(bl3_save.character_data.eridium, 1000000879);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 1);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 0);
        assert_eq!(first_playthrough.current_map, "Sanctuary");
        assert_eq!(first_playthrough.active_missions, vec!["Fire in the Sky",]);
        assert_eq!(first_playthrough.missions_completed.len(), 86);
        assert_eq!(first_playthrough.mission_milestones, vec!["Main Game"]);

        assert_eq!(
            bl3_save.character_data.unlockable_inventory_slots,
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
            bl3_save.character_data.sdu_slots,
            vec![
                SaveSduSlotData {
                    slot: SaveSduSlot::Backpack,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Sniper,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Shotgun,
                    current: 10,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Pistol,
                    current: 10,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Grenade,
                    current: 10,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Smg,
                    current: 10,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Ar,
                    current: 10,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Heavy,
                    current: 13,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.ammo_pools,
            vec![
                AmmoPoolData {
                    ammo: Ammo::Grenade,
                    current: 11,
                },
                AmmoPoolData {
                    ammo: Ammo::Pistol,
                    current: 1000,
                },
                AmmoPoolData {
                    ammo: Ammo::Shotgun,
                    current: 240,
                },
                AmmoPoolData {
                    ammo: Ammo::Smg,
                    current: 1800,
                },
                AmmoPoolData {
                    ammo: Ammo::Ar,
                    current: 662,
                },
                AmmoPoolData {
                    ammo: Ammo::Sniper,
                    current: 144,
                },
                AmmoPoolData {
                    ammo: Ammo::Heavy,
                    current: 36,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.challenge_milestones,
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
            bl3_save.character_data.vehicle_stats,
            vec![
                VehicleStats {
                    name: VehicleName::Outrunner,
                    chassis_count: 3,
                    total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                    parts_count: 8,
                    total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                    skins_count: 13,
                    total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
                },
                VehicleStats {
                    name: VehicleName::Jetbeast,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                    parts_count: 0,
                    total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                    skins_count: 0,
                    total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
                },
                VehicleStats {
                    name: VehicleName::Technical,
                    chassis_count: 3,
                    total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                    parts_count: 7,
                    total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                    skins_count: 16,
                    total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
                },
                VehicleStats {
                    name: VehicleName::Cyclone,
                    chassis_count: 3,
                    total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                    parts_count: 5,
                    total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                    skins_count: 12,
                    total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
                },
            ]
        );
    }

    #[test]
    fn test_from_data_ps4_3() {
        let mut save_file_data =
            fs::read("./test_files/310ps4.sav").expect("failed to read test_file");
        let bl3_save = Bl3Save::from_bytes(&mut save_file_data, HeaderType::Ps4Save)
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
        assert_eq!(bl3_save.character_data.player_class, PlayerClass::Operative);
        assert_eq!(bl3_save.character_data.character.experience_points, 5714393);
        assert_eq!(bl3_save.character_data.player_level, 60);
        assert_eq!(bl3_save.character_data.guardian_rank, 0);
        assert_eq!(bl3_save.character_data.money, 100000000);
        assert_eq!(bl3_save.character_data.eridium, 0);
        assert_eq!(bl3_save.character_data.character.playthroughs_completed, 0);

        let first_playthrough = bl3_save
            .character_data
            .playthroughs
            .get(0)
            .expect("failed to read first playthrough");
        assert_eq!(first_playthrough.mayhem_level, 0);
        assert_eq!(first_playthrough.mayhem_random_seed, 20046);
        assert_eq!(first_playthrough.current_map, "Sanctuary");
        assert_eq!(
            first_playthrough.active_missions,
            vec!["Hostile Takeover", "The Handsome Jackpot"]
        );
        assert_eq!(first_playthrough.missions_completed.len(), 5);
        assert_eq!(first_playthrough.mission_milestones.len(), 0);

        assert_eq!(
            bl3_save.character_data.unlockable_inventory_slots,
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
            bl3_save.character_data.sdu_slots,
            vec![
                SaveSduSlotData {
                    slot: SaveSduSlot::Backpack,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Sniper,
                    current: 13,
                    max: 13,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Shotgun,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Pistol,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Grenade,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Smg,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Ar,
                    current: 8,
                    max: 10,
                },
                SaveSduSlotData {
                    slot: SaveSduSlot::Heavy,
                    current: 13,
                    max: 13,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.ammo_pools,
            vec![
                AmmoPoolData {
                    ammo: Ammo::Grenade,
                    current: 3,
                },
                AmmoPoolData {
                    ammo: Ammo::Pistol,
                    current: 145,
                },
                AmmoPoolData {
                    ammo: Ammo::Shotgun,
                    current: 80,
                },
                AmmoPoolData {
                    ammo: Ammo::Smg,
                    current: 360,
                },
                AmmoPoolData {
                    ammo: Ammo::Ar,
                    current: 280,
                },
                AmmoPoolData {
                    ammo: Ammo::Sniper,
                    current: 48,
                },
                AmmoPoolData {
                    ammo: Ammo::Heavy,
                    current: 0,
                },
            ]
        );

        assert_eq!(
            bl3_save.character_data.challenge_milestones,
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
            bl3_save.character_data.vehicle_stats,
            vec![
                VehicleStats {
                    name: VehicleName::Outrunner,
                    chassis_count: 1,
                    total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                    parts_count: 4,
                    total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                    skins_count: 7,
                    total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
                },
                VehicleStats {
                    name: VehicleName::Jetbeast,
                    chassis_count: 2,
                    total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                    parts_count: 3,
                    total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                    skins_count: 1,
                    total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
                },
                VehicleStats {
                    name: VehicleName::Technical,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                    parts_count: 5,
                    total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                    skins_count: 7,
                    total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
                },
                VehicleStats {
                    name: VehicleName::Cyclone,
                    chassis_count: 0,
                    total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                    parts_count: 3,
                    total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                    skins_count: 8,
                    total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
                },
            ]
        );
    }
}
