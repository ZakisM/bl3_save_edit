use std::str::FromStr;

use anyhow::{Context, Result};

use crate::bl3_save::ammo::{Ammo, AmmoPoolData};
use crate::bl3_save::challenge_data::ChallengeData;
use crate::bl3_save::inventory_slot::{InventorySlot, InventorySlotData};
use crate::bl3_save::player_class::PlayerClass;
use crate::bl3_save::sdu::{SduSlot, SduSlotData};
use crate::bl3_save::util::{currency_amount_from_character, experience_to_level, read_playthroughs, Currency, IMPORTANT_CHALLENGES};
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
}

impl CharacterData {
    pub async fn from_character(character: Character) -> Result<Self> {
        let player_class = PlayerClass::from_str(
            character
                .player_class_data
                .as_ref()
                .map(|p| p.player_class_path.as_str())
                .context("failed to read player_class")?,
        )?;
        let player_level = experience_to_level(&character.experience_points)?;
        let guardian_rank = character
            .guardian_rank_character_data
            .as_ref()
            .map(|g| g.guardian_rank)
            .context("could not read character guardian_rank")?;
        let money = currency_amount_from_character(&character, &Currency::Money)?;
        let eridium = currency_amount_from_character(&character, &Currency::Eridium)?;
        let playthroughs = read_playthroughs(&character).await?;
        let unlockable_inventory_slots = character
            .equipped_inventory_list
            .iter()
            .map::<Result<_>, _>(|i| {
                Ok(InventorySlotData {
                    slot: InventorySlot::from_str(&i.slot_data_path)?,
                    unlocked: i.enabled,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let sdu_slots = character
            .sdu_list
            .iter()
            .map::<Result<_>, _>(|s| {
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
            .iter()
            .filter(|rp| !rp.resource_path.to_lowercase().contains("eridium"))
            .map::<Result<_>, _>(|rp| {
                let ammo = Ammo::from_str(&rp.resource_path)?;

                Ok(AmmoPoolData { ammo, current: rp.amount })
            })
            .collect::<Result<Vec<_>>>()?;

        let challenge_milestones = IMPORTANT_CHALLENGES
            .iter()
            .filter(|[k, _]| {
                let k = k.to_lowercase();

                if k.contains("character") {
                    k.contains(&player_class.to_string().to_lowercase())
                } else {
                    true
                }
            })
            .map::<Result<_>, _>(|[k, v]| {
                let unlocked = character
                    .challenge_data
                    .iter()
                    .find(|cd| cd.challenge_class_path.to_lowercase().contains(&k.to_lowercase()))
                    .map(|cd| cd.currently_completed)
                    .context("failed to read challenge_milestones")?;

                Ok(ChallengeData {
                    challenge: v.to_string(),
                    unlocked,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        dbg!(&character.vehicles_unlocked_data);

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
        })
    }
}

#[derive(Debug)]
pub struct Playthrough {
    pub mayhem_level: i32,
    pub mayhem_random_seed: i32,
    pub current_map: String,
    pub active_missions: Vec<String>,
    pub missions_completed: Vec<String>,
    pub mission_milestones: Vec<String>,
}
