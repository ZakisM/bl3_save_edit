use strum::{Display, EnumMessage, EnumString};

#[derive(Clone, Copy, Debug, Display, EnumString, EnumMessage, Eq, PartialEq, Ord, PartialOrd)]
pub enum PlayerClass {
    #[strum(
        serialize = "/Game/PlayerCharacters/Beastmaster/PlayerClassId_Beastmaster.PlayerClassId_Beastmaster",
        to_string = "Beastmaster"
    )]
    BeastMaster,
    #[strum(
        serialize = "/Game/PlayerCharacters/Gunner/PlayerClassId_Gunner.PlayerClassId_Gunner",
        to_string = "Gunner"
    )]
    Gunner,
    #[strum(
        serialize = "/Game/PlayerCharacters/Operative/PlayerClassId_Operative.PlayerClassId_Operative",
        to_string = "Operative"
    )]
    Operative,
    #[strum(
        serialize = "/Game/PlayerCharacters/SirenBrawler/PlayerClassId_Siren.PlayerClassId_Siren",
        to_string = "Siren"
    )]
    Siren,
}

impl Default for PlayerClass {
    fn default() -> Self {
        Self::BeastMaster
    }
}

impl PlayerClass {
    pub const ALL: [PlayerClass; 4] = [
        PlayerClass::BeastMaster,
        PlayerClass::Gunner,
        PlayerClass::Operative,
        PlayerClass::Siren,
    ];
}
