use anyhow::{Context, Result};

use crate::bl3_save::player_class::PlayerClass;
use crate::bl3_save::util::{
    currency_amount_from_character, experience_to_level, read_playthroughs, Currency,
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
    // pub unlockable_inventory_slots: Vec<InventorySlot>
}

impl CharacterData {
    pub fn from_character(character: Character) -> Result<Self> {
        let player_class = PlayerClass::from_character(&character)?;
        let player_level = experience_to_level(&character.experience_points)?;
        let guardian_rank = character
            .guardian_rank_character_data
            .as_ref()
            .map(|g| g.guardian_rank)
            .context("could not read character guardian_rank")?;
        let money = currency_amount_from_character(&character, &Currency::Money)?;
        let eridium = currency_amount_from_character(&character, &Currency::Eridium)?;
        let playthroughs = read_playthroughs(&character)?;

        Ok(Self {
            character,
            player_class,
            player_level,
            guardian_rank,
            money,
            eridium,
            playthroughs,
        })
    }
}

#[derive(Debug)]
pub struct Playthrough {
    mayhem_level: i32,
    mayhem_random_seed: i32,
    current_map: String,
    active_missions: Vec<String>,
    missions_completed: i32,
    mission_milestones: Vec<String>,
}

#[derive(Debug)]
pub struct InventorySlot {
    name: String,
    unlocked: bool,
}
