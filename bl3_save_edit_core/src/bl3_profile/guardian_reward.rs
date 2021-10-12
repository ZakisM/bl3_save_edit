use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct GuardianRewardData {
    pub reward: GuardianReward,
    pub current: i32,
    pub max: i32,
}

#[derive(
    Debug, Display, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum GuardianReward {
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_Accuracy.GuardianReward_Accuracy",
        to_string = "Accuracy"
    )]
    Accuracy,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ActionSkillCooldown.GuardianReward_ActionSkillCooldown",
        to_string = "Action Skill Cooldown"
    )]
    ActionSkillCooldown,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_CriticalDamage.GuardianReward_CriticalDamage",
        to_string = "Critical Damage"
    )]
    CriticalDamage,
    #[strum(
        serialize = "/Game/PatchDLC/Hibiscus/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ElementalDamage.GuardianReward_ElementalDamage",
        to_string = "Elemental Damage"
    )]
    ElementalDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_FFYLDuration.GuardianReward_FFYLDuration",
        to_string = "FFYL Duration"
    )]
    FFYLDuration,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_FFYLMovementSpeed.GuardianReward_FFYLMovementSpeed",
        to_string = "FFYL Movement Speed"
    )]
    FFYLMovementSpeed,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_GrenadeDamage.GuardianReward_GrenadeDamage",
        to_string = "Grenade Damage"
    )]
    GrenadeDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_GunDamage.GuardianReward_GunDamage",
        to_string = "Gun Damage"
    )]
    GunDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_GunFireRate.GuardianReward_GunFireRate",
        to_string = "Gun Fire Rate"
    )]
    GunFireRate,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_MaxHealth.GuardianReward_MaxHealth",
        to_string = "Max Health"
    )]
    MaxHealth,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_MeleeDamage.GuardianReward_MeleeDamage",
        to_string = "Melee Damage"
    )]
    MeleeDamage,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_RarityRate.GuardianReward_RarityRate",
        to_string = "Luck"
    )]
    RarityRate,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_RecoilReduction.GuardianReward_RecoilReduction",
        to_string = "Recoil Reduction"
    )]
    RecoilReduction,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ReloadSpeed.GuardianReward_ReloadSpeed",
        to_string = "Reload Speed"
    )]
    ReloadSpeed,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ShieldCapacity.GuardianReward_ShieldCapacity",
        to_string = "Shield Capacity"
    )]
    ShieldCapacity,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ShieldRechargeDelay.GuardianReward_ShieldRechargeDelay",
        to_string = "Shield Recharge Delay"
    )]
    ShieldRechargeDelay,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_ShieldRechargeRate.GuardianReward_ShieldRechargeRate",
        to_string = "Shield Recharge Rate"
    )]
    ShieldRechargeRate,
    #[strum(
        serialize = "/Game/PlayerCharacters/_Shared/_Design/GuardianRank/GuardianReward_VehicleDamage.GuardianReward_VehicleDamage",
        to_string = "Vehicle Damage"
    )]
    VehicleDamage,
}

impl std::default::Default for GuardianReward {
    fn default() -> Self {
        Self::Accuracy
    }
}
