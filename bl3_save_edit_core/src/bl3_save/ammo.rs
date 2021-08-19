use strum::{Display, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct AmmoPoolData {
    pub pool: AmmoPool,
    pub current: usize,
}

#[derive(Debug, Display, EnumString, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum AmmoPool {
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Grenade.Resource_Ammo_Grenade",
        to_string = "Grenade"
    )]
    Grenade,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Pistol.Resource_Ammo_Pistol",
        to_string = "Pistol"
    )]
    Pistol,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Shotgun.Resource_Ammo_Shotgun",
        to_string = "Shotgun"
    )]
    Shotgun,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_SMG.Resource_Ammo_SMG",
        to_string = "SMG"
    )]
    Smg,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_AssaultRifle.Resource_Ammo_AssaultRifle",
        to_string = "AR"
    )]
    Ar,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Sniper.Resource_Ammo_Sniper",
        to_string = "Sniper"
    )]
    Sniper,
    #[strum(
        serialize = "/Game/GameData/Weapons/Ammo/Resource_Ammo_Heavy.Resource_Ammo_Heavy",
        to_string = "Heavy"
    )]
    Heavy,
}

impl std::default::Default for AmmoPool {
    fn default() -> Self {
        Self::Grenade
    }
}
