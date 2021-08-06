use std::str::FromStr;

use anyhow::{Context, Result};
use derivative::Derivative;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use strum::IntoEnumIterator;

use crate::bl3_save::ammo::{Ammo, AmmoPoolData};
use crate::bl3_save::bl3_item::Bl3Item;
use crate::bl3_save::challenge_data::Challenge;
use crate::bl3_save::challenge_data::ChallengeData;
use crate::bl3_save::inventory_slot::{InventorySlot, InventorySlotData};
use crate::bl3_save::models::{Currency, VisitedTeleporter};
use crate::bl3_save::player_class::PlayerClass;
use crate::bl3_save::playthrough::Playthrough;
use crate::bl3_save::sdu::{SaveSduSlot, SaveSduSlotData};
use crate::bl3_save::util::{
    currency_amount_from_character, experience_to_level, IMPORTANT_CHALLENGES,
};
use crate::game_data::{
    GameDataKv, PROFILE_ECHO_THEMES, PROFILE_ECHO_THEMES_DEFAULTS, PROFILE_HEADS,
    PROFILE_HEADS_DEFAULTS, PROFILE_SKINS, PROFILE_SKINS_DEFAULTS, VEHICLE_CHASSIS_CYCLONE,
    VEHICLE_CHASSIS_JETBEAST, VEHICLE_CHASSIS_OUTRUNNER, VEHICLE_CHASSIS_TECHNICAL,
    VEHICLE_PARTS_CYCLONE, VEHICLE_PARTS_JETBEAST, VEHICLE_PARTS_OUTRUNNER,
    VEHICLE_PARTS_TECHNICAL, VEHICLE_SKINS_CYCLONE, VEHICLE_SKINS_JETBEAST,
    VEHICLE_SKINS_OUTRUNNER, VEHICLE_SKINS_TECHNICAL,
};
use crate::protos::oak_save::Character;
use crate::vehicle_data::{VehicleName, VehicleStats};

#[derive(Derivative)]
#[derivative(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct CharacterData {
    #[derivative(PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    pub character: Character,
    pub player_class: PlayerClass,
    pub player_level: i32,
    pub guardian_rank: i32,
    pub head_skin_selected: GameDataKv,
    pub character_skin_selected: GameDataKv,
    pub echo_theme_selected: GameDataKv,
    pub money: i32,
    pub eridium: i32,
    pub playthroughs: Vec<Playthrough>,
    pub unlockable_inventory_slots: Vec<InventorySlotData>,
    pub sdu_slots: Vec<SaveSduSlotData>,
    pub ammo_pools: Vec<AmmoPoolData>,
    pub challenge_milestones: Vec<ChallengeData>,
    pub vehicle_stats: Vec<VehicleStats>,
    pub inventory_items: Vec<Bl3Item>,
}

impl CharacterData {
    pub fn from_character(character: Character) -> Result<Self> {
        let player_class = PlayerClass::from_str(
            character
                .player_class_data
                .as_ref()
                .map(|p| p.player_class_path.as_str())
                .context("failed to read player class")?,
        )?;
        let player_level = experience_to_level(character.experience_points)?;
        let guardian_rank = character
            .guardian_rank_character_data
            .as_ref()
            .map(|g| g.guardian_rank)
            .unwrap_or(0);

        let available_head_skins = PROFILE_HEADS_DEFAULTS
            .par_iter()
            .chain(PROFILE_HEADS.par_iter())
            .cloned()
            .filter(|h| {
                h.ident
                    .to_lowercase()
                    .contains(&player_class.to_string().to_lowercase())
            })
            .collect::<Vec<_>>();

        let head_skin_selected = available_head_skins
            .par_iter()
            .cloned()
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs == s.ident)
            })
            .unwrap_or(available_head_skins[0]);

        let available_character_skins = PROFILE_SKINS_DEFAULTS
            .par_iter()
            .chain(PROFILE_SKINS.par_iter())
            .cloned()
            .filter(|h| {
                h.ident
                    .to_lowercase()
                    .contains(&player_class.to_string().to_lowercase())
            })
            .collect::<Vec<_>>();

        let character_skin_selected = available_character_skins
            .par_iter()
            .cloned()
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs == s.ident)
            })
            .unwrap_or(available_character_skins[0]);

        let echo_theme_selected = PROFILE_ECHO_THEMES_DEFAULTS
            .par_iter()
            .chain(PROFILE_ECHO_THEMES.par_iter())
            .cloned()
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs == s.ident)
            })
            .unwrap_or(PROFILE_ECHO_THEMES_DEFAULTS[0]);

        let money = currency_amount_from_character(&character, &Currency::Money);
        let eridium = currency_amount_from_character(&character, &Currency::Eridium);
        let playthroughs = Playthrough::playthroughs_from_character(&character)?;
        let mut unlockable_inventory_slots = character
            .equipped_inventory_list
            .par_iter()
            .map(|i| {
                Ok(InventorySlotData {
                    slot: InventorySlot::from_str(&i.slot_data_path).with_context(|| {
                        format!("failed to read inventory slot: {}", &i.slot_data_path)
                    })?,
                    unlocked: i.enabled,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        unlockable_inventory_slots.par_sort();

        let mut sdu_slots = character
            .sdu_list
            .par_iter()
            .map(|s| {
                let slot = SaveSduSlot::from_str(&s.sdu_data_path).with_context(|| {
                    format!("failed to read save sdu slot: {}", &s.sdu_data_path)
                })?;
                let max = slot.maximum();

                Ok(SaveSduSlotData {
                    slot,
                    current: s.sdu_level,
                    max,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        // make sure that we include all sdu slots that might not be in our save
        SaveSduSlot::iter().for_each(|sdu| {
            let contains_sdu_slot = sdu_slots.par_iter().any(|save_sdu| {
                std::mem::discriminant(&sdu) == std::mem::discriminant(&save_sdu.slot)
            });

            if !contains_sdu_slot {
                sdu_slots.push(SaveSduSlotData {
                    slot: sdu.to_owned(),
                    current: 0,
                    max: sdu.maximum(),
                })
            }
        });

        sdu_slots.par_sort();

        let mut ammo_pools = character
            .resource_pools
            .par_iter()
            .filter(|rp| !rp.resource_path.contains("Eridium"))
            .map(|rp| {
                let ammo = Ammo::from_str(&rp.resource_path)
                    .with_context(|| format!("failed to read ammo: {}", &rp.resource_path))?;

                Ok(AmmoPoolData {
                    ammo,
                    current: rp.amount as usize,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        ammo_pools.par_sort();

        let mut challenge_milestones = IMPORTANT_CHALLENGES
            .par_iter()
            .filter(|[k, _]| {
                let k = k.to_lowercase();

                if k.contains("character") {
                    k.contains(&player_class.to_string().to_lowercase())
                } else {
                    true
                }
            })
            .map(|[k, v]| {
                let unlocked = character
                    .challenge_data
                    .par_iter()
                    .find_first(|cd| cd.challenge_class_path.contains(k))
                    .map(|cd| cd.currently_completed)
                    .context("failed to read challenge milestones")?;

                Ok(ChallengeData {
                    challenge: Challenge::from_str(v)
                        .with_context(|| format!("failed to read challenge: {}", v))?,
                    unlocked,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        challenge_milestones.par_sort();

        let mut outrunner_chassis = 0;
        let mut jetbeast_chassis = 0;
        let mut technical_chassis = 0;
        let mut cyclone_chassis = 0;

        character.vehicles_unlocked_data.iter().for_each(|vu| {
            let vu = &vu.asset_path;
            let vu = &vu.as_str();

            match vu {
                vu if VEHICLE_CHASSIS_OUTRUNNER.contains(vu) => {
                    outrunner_chassis += 1;
                }
                vu if VEHICLE_CHASSIS_JETBEAST.contains(vu) => {
                    jetbeast_chassis += 1;
                }
                vu if VEHICLE_CHASSIS_TECHNICAL.contains(vu) => {
                    technical_chassis += 1;
                }
                vu if VEHICLE_CHASSIS_CYCLONE.contains(vu) => {
                    cyclone_chassis += 1;
                }
                _ => (),
            };
        });

        let mut outrunner_parts = 0;
        let mut jetbeast_parts = 0;
        let mut technical_parts = 0;
        let mut cyclone_parts = 0;

        let mut outrunner_skins = 0;
        let mut jetbeast_skins = 0;
        let mut technical_skins = 0;
        let mut cyclone_skins = 0;

        character.vehicle_parts_unlocked.iter().for_each(|vp| {
            let vp = vp;
            let vp = &vp.as_str();

            match vp {
                vp if VEHICLE_PARTS_OUTRUNNER.contains(vp) => {
                    outrunner_parts += 1;
                }
                vp if VEHICLE_PARTS_JETBEAST.contains(vp) => {
                    jetbeast_parts += 1;
                }
                vp if VEHICLE_PARTS_TECHNICAL.contains(vp) => {
                    technical_parts += 1;
                }
                vp if VEHICLE_PARTS_CYCLONE.contains(vp) => {
                    cyclone_parts += 1;
                }
                vp if VEHICLE_SKINS_OUTRUNNER.contains(vp) => {
                    outrunner_skins += 1;
                }
                vp if VEHICLE_SKINS_JETBEAST.contains(vp) => {
                    jetbeast_skins += 1;
                }
                vp if VEHICLE_SKINS_TECHNICAL.contains(vp) => {
                    technical_skins += 1;
                }
                vp if VEHICLE_SKINS_CYCLONE.contains(vp) => {
                    cyclone_skins += 1;
                }
                _ => (),
            };
        });

        let vehicle_stats = vec![
            VehicleStats {
                name: VehicleName::Outrunner,
                chassis_count: outrunner_chassis,
                total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                parts_count: outrunner_parts,
                total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                skins_count: outrunner_skins,
                total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
            },
            VehicleStats {
                name: VehicleName::Jetbeast,
                chassis_count: jetbeast_chassis,
                total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                parts_count: jetbeast_parts,
                total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                skins_count: jetbeast_skins,
                total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
            },
            VehicleStats {
                name: VehicleName::Technical,
                chassis_count: technical_chassis,
                total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                parts_count: technical_parts,
                total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                skins_count: technical_skins,
                total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
            },
            VehicleStats {
                name: VehicleName::Cyclone,
                chassis_count: cyclone_chassis,
                total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                parts_count: cyclone_parts,
                total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                skins_count: cyclone_skins,
                total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
            },
        ];

        let inventory_items = character
            .inventory_items
            .par_iter()
            .filter_map(|i| Bl3Item::from_serial_number(i.item_serial_number.clone()).ok())
            .collect::<Vec<_>>();

        Ok(Self {
            character,
            player_class,
            player_level,
            guardian_rank,
            head_skin_selected,
            character_skin_selected,
            echo_theme_selected,
            money,
            eridium,
            playthroughs,
            unlockable_inventory_slots,
            sdu_slots,
            ammo_pools,
            challenge_milestones,
            vehicle_stats,
            inventory_items,
        })
    }

    pub fn set_head_skin_selected(&mut self, head_skin_selected: &GameDataKv) {
        let curr_head_skin_selected = self.head_skin_selected.ident;
        let head_skin_selected_id = head_skin_selected.ident.to_owned();

        if let Some(curr_head) = self
            .character
            .selected_customizations
            .iter_mut()
            .find(|curr| curr_head_skin_selected == *curr)
        {
            *curr_head = head_skin_selected_id;
        } else {
            self.character
                .selected_customizations
                .push(head_skin_selected_id)
        }
    }

    pub fn set_active_travel_stations(
        &mut self,
        _playthrough_index: usize,
        _visited_teleporters_list: &[VisitedTeleporter],
    ) {
        //TODO: Find a save with every location and map everything below...

        // let mission_list = self.character.mission_playthroughs_data.get_mut(0);
        //
        // if let Some(mission_list) = mission_list {
        //     mission_list.mission_list.push(MissionStatusPlayerSaveGameData {
        //         status: MissionStatusPlayerSaveGameData_MissionState::MS_Complete,
        //         has_been_viewed_in_log: false,
        //         objectives_progress: vec![1, 1, 1, 0, 1, 1],
        //         mission_class_path: "/Game/Missions/Side/Slaughters/TechSlaughter/Mission_TechSlaughterDiscovery.Mission_TechSlaughterDiscovery_C".to_string(),
        //         active_objective_set_path: "/Game/Missions/Side/Slaughters/TechSlaughter/Mission_TechSlaughterDiscovery.Set_TalkToNPC_ObjectiveSet".to_string(),
        //         dlc_package_id: 0,
        //         kickoff_played: true,
        //         league_instance: 0,
        //         unknown_fields: Default::default(),
        //         cached_size: Default::default(),
        //     });
        //
        //     dbg!(&mission_list.mission_list);
        // }
        //
        // let curr_active_travel_stations = &mut self
        //     .character
        //     .active_travel_stations_for_playthrough
        //     .get_mut(playthrough_index)
        //     .expect("failed to read current active travel stations for playthrough: ")
        //     .active_travel_stations;
        //
        // curr_active_travel_stations.push(ActiveFastTravelSaveData {
        //     active_travel_station_name:
        //         "/Game/GameData/FastTravel/FTS_TechSlaughterDropPod.FTS_TechSlaughterDropPod"
        //             .to_owned(),
        //     blacklisted: false,
        //     unknown_fields: Default::default(),
        //     cached_size: Default::default(),
        // });
        //
        // let discovery_data = &mut self.character.discovery_data;
        //
        // if let Some(discovery_data) = discovery_data.as_mut() {
        //     discovery_data
        //         .discovered_level_info
        //         .push(DiscoveredLevelInfo {
        //             discovered_level_name: "/Game/Maps/Slaughters/TechSlaughter/TechSlaughter_P"
        //                 .to_string(),
        //             //the index of playthrough + 1 i think
        //             discovered_playthroughs: 1,
        //             discovered_area_info: RepeatedField::from_vec(vec![DiscoveredAreaInfo {
        //                 discovered_area_name: "TECHSLAUGHTER_PWDA_2".to_string(),
        //                 //the index of playthrough + 1 i think
        //                 discovered_playthroughs: 1,
        //                 unknown_fields: Default::default(),
        //                 cached_size: Default::default(),
        //             }]),
        //             unknown_fields: Default::default(),
        //             cached_size: Default::default(),
        //         });
        // }
        //
        // let challenge_data = &mut self.character.challenge_data;
        //
        // let challenge_we_want = challenge_data
        //     .iter_mut()
        //     .find(|cd| cd.challenge_class_path == "/Game/GameData/Challenges/Discovery/Slaughter_Tech/Challenge_Discovery_TechSlaughter1.Challenge_Discovery_TechSlaughter1_C");
        //
        // if let Some(challenge_we_want) = challenge_we_want {
        //     challenge_we_want.completed_count = 1;
        //     challenge_we_want.currently_completed = true;
        // }
        //
        // let challenge_we_want = challenge_data
        //     .iter_mut()
        //     .find(|cd| cd.challenge_class_path == "/Game/GameData/Challenges/FastTravel/Challenge_FastTravel_TechSlaughter1.Challenge_FastTravel_TechSlaughter1_C");
        //
        // if let Some(challenge_we_want) = challenge_we_want {
        //     challenge_we_want.completed_count = 1;
        //     challenge_we_want.currently_completed = true;
        // }

        let save_name = format!("{}-out.txt", self.character.save_game_id);
        let data = format!("{:#?}", self.character);

        std::fs::write(save_name, data).unwrap();

        // visited_teleporters_list.iter().for_each(|vt| {
        //     if vt.visited
        //         && !curr_active_travel_stations
        //             .iter()
        //             .any(|ats| ats.active_travel_station_name.to_lowercase() == vt.game_data.ident)
        //     {
        //         // println!("Adding: {}", vt.game_data.ident);
        //
        //         // curr_active_travel_stations.push(ActiveFastTravelSaveData {
        //         //     active_travel_station_name: vt.game_data.ident.to_owned(),
        //         //     blacklisted: false,
        //         //     unknown_fields: Default::default(),
        //         //     cached_size: Default::default(),
        //         // });
        //     } else if !vt.visited {
        //         if let Some(curr_station) = curr_active_travel_stations.iter().position(|ats| {
        //             ats.active_travel_station_name.to_lowercase() == vt.game_data.ident
        //         }) {
        //             // println!("Removing: {}", vt.game_data.ident);
        //
        //             // curr_active_travel_stations.remove(curr_station);
        //         }
        //     }
        // })
    }
}
