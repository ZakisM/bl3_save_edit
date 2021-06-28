use std::str::FromStr;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::bl3_save::ammo::{Ammo, AmmoPoolData};
use crate::bl3_save::inventory_slot::{InventorySlot, InventorySlotData};
use crate::bl3_save::models::{ChallengeData, Currency, Playthrough, VehicleStats};
use crate::bl3_save::player_class::PlayerClass;
use crate::bl3_save::sdu::{SduSlot, SduSlotData};
use crate::bl3_save::util::{currency_amount_from_character, experience_to_level, read_playthroughs, IMPORTANT_CHALLENGES};
use crate::game_data::{
    VEHICLE_CHASSIS_CYCLONE, VEHICLE_CHASSIS_JETBEAST, VEHICLE_CHASSIS_OUTRUNNER, VEHICLE_CHASSIS_TECHNICAL, VEHICLE_PARTS_CYCLONE,
    VEHICLE_PARTS_JETBEAST, VEHICLE_PARTS_OUTRUNNER, VEHICLE_PARTS_TECHNICAL, VEHICLE_SKINS_CYCLONE, VEHICLE_SKINS_JETBEAST, VEHICLE_SKINS_OUTRUNNER,
    VEHICLE_SKINS_TECHNICAL,
};
use crate::protos::oak_save::Character;

#[derive(Debug)]
pub struct CharacterData {
    pub character: Character,
    pub player_class: PlayerClass,
    pub player_level: i32,
    pub guardian_rank: i32,
    pub money: i32,
    pub eridium: i32,
    pub playthroughs: Vec<Playthrough>,
    pub unlockable_inventory_slots: Vec<InventorySlotData>,
    pub sdu_slots: Vec<SduSlotData>,
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
        let player_level = experience_to_level(&character.experience_points)?;
        let guardian_rank = character.guardian_rank_character_data.as_ref().map(|g| g.guardian_rank).unwrap_or(0);
        let money = currency_amount_from_character(&character, &Currency::Money);
        let eridium = currency_amount_from_character(&character, &Currency::Eridium);
        let playthroughs = read_playthroughs(&character)?;
        let unlockable_inventory_slots = character
            .equipped_inventory_list
            .par_iter()
            .map(|i| {
                Ok(InventorySlotData {
                    slot: InventorySlot::from_str(&i.slot_data_path)?,
                    unlocked: i.enabled,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let sdu_slots = character
            .sdu_list
            .par_iter()
            .map(|s| {
                let slot = SduSlot::from_str(&s.sdu_data_path)?;
                let max = slot.maximum();

                Ok(SduSlotData {
                    slot,
                    current: s.sdu_level,
                    max,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let ammo_pools = character
            .resource_pools
            .par_iter()
            .filter(|rp| !rp.resource_path.to_lowercase().contains("eridium"))
            .map(|rp| {
                let ammo = Ammo::from_str(&rp.resource_path)?;

                Ok(AmmoPoolData { ammo, current: rp.amount })
            })
            .collect::<Result<Vec<_>>>()?;

        let challenge_milestones = IMPORTANT_CHALLENGES
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
                    .find_first(|cd| cd.challenge_class_path.to_lowercase().contains(&k.to_lowercase()))
                    .map(|cd| cd.currently_completed)
                    .context("failed to read challenge milestones")?;

                Ok(ChallengeData {
                    challenge: v.to_string(),
                    unlocked,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let outrunner_chassis_c = AtomicUsize::new(0);
        let jetbeast_chassis_c = AtomicUsize::new(0);
        let technical_chassis_c = AtomicUsize::new(0);
        let cyclone_chassis_c = AtomicUsize::new(0);

        character.vehicles_unlocked_data.par_iter().for_each(|vu| {
            let vu = vu.asset_path.to_lowercase();
            let vu = &vu.as_str();

            match vu {
                vu if VEHICLE_CHASSIS_OUTRUNNER.contains(vu) => outrunner_chassis_c.fetch_add(1, Ordering::Release),
                vu if VEHICLE_CHASSIS_JETBEAST.contains(vu) => jetbeast_chassis_c.fetch_add(1, Ordering::Release),
                vu if VEHICLE_CHASSIS_TECHNICAL.contains(vu) => technical_chassis_c.fetch_add(1, Ordering::Release),
                vu if VEHICLE_CHASSIS_CYCLONE.contains(vu) => cyclone_chassis_c.fetch_add(1, Ordering::Release),
                _ => 0,
            };
        });

        let outrunner_parts_c = AtomicUsize::new(0);
        let jetbeast_parts_c = AtomicUsize::new(0);
        let technical_parts_c = AtomicUsize::new(0);
        let cyclone_parts_c = AtomicUsize::new(0);

        let outrunner_skins_c = AtomicUsize::new(0);
        let jetbeast_skins_c = AtomicUsize::new(0);
        let technical_skins_c = AtomicUsize::new(0);
        let cyclone_skins_c = AtomicUsize::new(0);

        character.vehicle_parts_unlocked.par_iter().for_each(|vp| {
            let vp = vp.to_lowercase();
            let vp = &vp.as_str();

            match vp {
                vp if VEHICLE_PARTS_OUTRUNNER.contains(vp) => outrunner_parts_c.fetch_add(1, Ordering::Release),
                vp if VEHICLE_PARTS_JETBEAST.contains(vp) => jetbeast_parts_c.fetch_add(1, Ordering::Release),
                vp if VEHICLE_PARTS_TECHNICAL.contains(vp) => technical_parts_c.fetch_add(1, Ordering::Release),
                vp if VEHICLE_PARTS_CYCLONE.contains(vp) => cyclone_parts_c.fetch_add(1, Ordering::Release),
                vp if VEHICLE_SKINS_OUTRUNNER.contains(vp) => outrunner_skins_c.fetch_add(1, Ordering::Release),
                vp if VEHICLE_SKINS_JETBEAST.contains(vp) => jetbeast_skins_c.fetch_add(1, Ordering::Release),
                vp if VEHICLE_SKINS_TECHNICAL.contains(vp) => technical_skins_c.fetch_add(1, Ordering::Release),
                vp if VEHICLE_SKINS_CYCLONE.contains(vp) => cyclone_skins_c.fetch_add(1, Ordering::Release),
                _ => 0,
            };
        });

        let vehicle_stats = vec![
            VehicleStats {
                name: "Outrunner".to_string(),
                chassis_count: outrunner_chassis_c.load(Ordering::Acquire),
                total_chassis_count: VEHICLE_CHASSIS_OUTRUNNER.len(),
                parts_count: outrunner_parts_c.load(Ordering::Acquire),
                total_parts_count: VEHICLE_PARTS_OUTRUNNER.len(),
                skins_count: outrunner_skins_c.load(Ordering::Acquire),
                total_skins_count: VEHICLE_SKINS_OUTRUNNER.len(),
            },
            VehicleStats {
                name: "Jetbeast".to_string(),
                chassis_count: jetbeast_chassis_c.load(Ordering::Acquire),
                total_chassis_count: VEHICLE_CHASSIS_JETBEAST.len(),
                parts_count: jetbeast_parts_c.load(Ordering::Acquire),
                total_parts_count: VEHICLE_PARTS_JETBEAST.len(),
                skins_count: jetbeast_skins_c.load(Ordering::Acquire),
                total_skins_count: VEHICLE_SKINS_JETBEAST.len(),
            },
            VehicleStats {
                name: "Technical".to_string(),
                chassis_count: technical_chassis_c.load(Ordering::Acquire),
                total_chassis_count: VEHICLE_CHASSIS_TECHNICAL.len(),
                parts_count: technical_parts_c.load(Ordering::Acquire),
                total_parts_count: VEHICLE_PARTS_TECHNICAL.len(),
                skins_count: technical_skins_c.load(Ordering::Acquire),
                total_skins_count: VEHICLE_SKINS_TECHNICAL.len(),
            },
            VehicleStats {
                name: "Cyclone".to_string(),
                chassis_count: cyclone_chassis_c.load(Ordering::Acquire),
                total_chassis_count: VEHICLE_CHASSIS_CYCLONE.len(),
                parts_count: cyclone_parts_c.load(Ordering::Acquire),
                total_parts_count: VEHICLE_PARTS_CYCLONE.len(),
                skins_count: cyclone_skins_c.load(Ordering::Acquire),
                total_skins_count: VEHICLE_SKINS_CYCLONE.len(),
            },
        ];

        Ok(Self {
            character,
            player_class,
            player_level,
            guardian_rank,
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
}
