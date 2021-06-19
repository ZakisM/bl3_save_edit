use anyhow::{bail, Result};
use strum::Display;

use crate::protos::oak_save::Character;

#[derive(Debug, Display)]
pub enum PlayerClass {
    BeastMaster,
    Gunner,
    Operative,
    Siren,
}

impl PlayerClass {
    pub fn from_character(character: &Character) -> Result<Self> {
        let character = match character.player_class_data.as_ref().map(|p| p.player_class_path.as_str()).unwrap_or("") {
            "/Game/PlayerCharacters/Beastmaster/PlayerClassId_Beastmaster.PlayerClassId_Beastmaster" => Self::BeastMaster,
            "/Game/PlayerCharacters/Gunner/PlayerClassId_Gunner.PlayerClassId_Gunner" => Self::Gunner,
            "/Game/PlayerCharacters/Operative/PlayerClassId_Operative.PlayerClassId_Operative" => Self::Operative,
            "/Game/PlayerCharacters/SirenBrawler/PlayerClassId_Siren.PlayerClassId_Siren" => Self::Siren,
            _ => bail!("could not find character"),
        };

        Ok(character)
    }
}
