use std::str::FromStr;

use anyhow::{Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;

use crate::bl3_save::ammo::{Ammo, AmmoPoolData};
use crate::bl3_save::challenge_data::Challenge;
use crate::bl3_save::challenge_data::ChallengeData;
use crate::bl3_save::inventory_slot::{InventorySlot, InventorySlotData};
use crate::bl3_save::models::{Currency, Playthrough};
use crate::bl3_save::player_class::PlayerClass;
use crate::bl3_save::sdu::{SaveSduSlot, SaveSduSlotData};
use crate::bl3_save::util::{
    currency_amount_from_character, experience_to_level, read_playthroughs, IMPORTANT_CHALLENGES,
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

#[derive(Debug, Clone, Default)]
pub struct CharacterData {
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
            .cloned()
            .chain(PROFILE_HEADS)
            .filter(|h| h.0 .0.contains(&player_class.to_string().to_lowercase()))
            .collect::<Vec<_>>();

        let head_skin_selected = available_head_skins
            .par_iter()
            .cloned()
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs.to_lowercase() == s.0 .0)
            })
            .unwrap_or(available_head_skins[0]);

        let available_character_skins = PROFILE_SKINS_DEFAULTS
            .par_iter()
            .cloned()
            .chain(PROFILE_SKINS)
            .filter(|h| h.0 .0.contains(&player_class.to_string().to_lowercase()))
            .collect::<Vec<_>>();

        let character_skin_selected = available_character_skins
            .par_iter()
            .cloned()
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs.to_lowercase() == s.0 .0)
            })
            .unwrap_or(available_character_skins[0]);

        let echo_theme_selected = PROFILE_ECHO_THEMES_DEFAULTS
            .par_iter()
            .cloned()
            .chain(PROFILE_ECHO_THEMES)
            .find_first(|s| {
                character
                    .selected_customizations
                    .par_iter()
                    .any(|cs| cs.to_lowercase() == s.0 .0)
            })
            .unwrap_or(PROFILE_ECHO_THEMES_DEFAULTS[0]);

        let money = currency_amount_from_character(&character, &Currency::Money);
        let eridium = currency_amount_from_character(&character, &Currency::Eridium);
        let playthroughs = read_playthroughs(&character)?;
        let mut unlockable_inventory_slots = character
            .equipped_inventory_list
            .par_iter()
            .map(|i| {
                Ok(InventorySlotData {
                    slot: InventorySlot::from_str(&i.slot_data_path)?,
                    unlocked: i.enabled,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        unlockable_inventory_slots.par_sort();

        let mut sdu_slots = character
            .sdu_list
            .par_iter()
            .map(|s| {
                let slot = SaveSduSlot::from_str(&s.sdu_data_path)?;
                let max = slot.maximum();

                Ok(SaveSduSlotData {
                    slot,
                    current: s.sdu_level,
                    max,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        sdu_slots.par_sort();

        let mut ammo_pools = character
            .resource_pools
            .par_iter()
            .filter(|rp| !rp.resource_path.to_lowercase().contains("eridium"))
            .map(|rp| {
                let ammo = Ammo::from_str(&rp.resource_path)?;

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
                    .find_first(|cd| {
                        cd.challenge_class_path
                            .to_lowercase()
                            .contains(&k.to_lowercase())
                    })
                    .map(|cd| cd.currently_completed)
                    .context("failed to read challenge milestones")?;

                Ok(ChallengeData {
                    challenge: Challenge::from_str(v)?,
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
            let vu = vu.asset_path.to_lowercase();
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
            let vp = vp.to_lowercase();
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
        })
    }

    pub fn set_head_skin_selected(&mut self, head_skin_selected: &GameDataKv) {
        let curr_head_skin_selected = self.head_skin_selected.0 .0;
        let head_skin_selected_id = head_skin_selected.0 .0.to_owned();

        if let Some(curr_head) = self
            .character
            .selected_customizations
            .iter_mut()
            .find(|curr| curr_head_skin_selected == curr.to_lowercase())
        {
            *curr_head = head_skin_selected_id;
        } else {
            self.character
                .selected_customizations
                .push(head_skin_selected_id)
        }
    }
}
