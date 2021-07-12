use strum::{Display, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct SaveSduSlotData {
    pub slot: SaveSduSlot,
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Display, EnumString, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum SaveSduSlot {
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Backpack.SDU_Backpack",
        to_string = "Backpack"
    )]
    Backpack,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_SniperRifle.SDU_SniperRifle",
        to_string = "Sniper"
    )]
    Sniper,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Shotgun.SDU_Shotgun",
        to_string = "Shotgun"
    )]
    Shotgun,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Pistol.SDU_Pistol",
        to_string = "Pistol"
    )]
    Pistol,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Grenade.SDU_Grenade",
        to_string = "Grenade"
    )]
    Grenade,
    #[strum(serialize = "/Game/Pickups/SDU/SDU_SMG.SDU_SMG", to_string = "SMG")]
    Smg,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_AssaultRifle.SDU_AssaultRifle",
        to_string = "AR"
    )]
    Ar,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_Heavy.SDU_Heavy",
        to_string = "Heavy"
    )]
    Heavy,
}

impl SaveSduSlot {
    pub fn maximum(&self) -> i32 {
        match *self {
            SaveSduSlot::Backpack | SaveSduSlot::Sniper | SaveSduSlot::Heavy => 13,
            SaveSduSlot::Shotgun
            | SaveSduSlot::Pistol
            | SaveSduSlot::Grenade
            | SaveSduSlot::Smg
            | SaveSduSlot::Ar => 10,
        }
    }
}
