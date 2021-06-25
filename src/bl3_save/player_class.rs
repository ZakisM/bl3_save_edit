use strum::{Display, EnumString};

#[derive(Debug, Display, EnumString)]
pub enum PlayerClass {
    #[strum(
        serialize = "/Game/PlayerCharacters/Beastmaster/PlayerClassId_Beastmaster.PlayerClassId_Beastmaster",
        to_string = "BeastMaster"
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
