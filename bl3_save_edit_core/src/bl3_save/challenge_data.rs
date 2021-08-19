use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct ChallengeData {
    pub challenge: Challenge,
    pub unlocked: bool,
}

#[derive(
    Debug, Display, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum Challenge {
    #[strum(
        serialize = "/Game/GameData/Challenges/Account/Challenge_VaultReward_ArtifactSlot.Challenge_VaultReward_ArtifactSlot_C",
        to_string = "Artifact Slot"
    )]
    ArtifactSlot,
    #[strum(
        serialize = "/Game/GameData/Challenges/Account/Challenge_VaultReward_Analyzer.Challenge_VaultReward_Analyzer_C",
        to_string = "Eridian Analyzer"
    )]
    EridianAnalyzer,
    #[strum(
        serialize = "/Game/GameData/Challenges/Account/Challenge_VaultReward_Resonator.Challenge_VaultReward_Resonator_C",
        to_string = "Eridian Resonator"
    )]
    EridianResonator,
    #[strum(
        serialize = "/Game/GameData/Challenges/Account/Challenge_VaultReward_Mayhem.Challenge_VaultReward_Mayhem_C",
        to_string = "Mayhem Mode"
    )]
    MayhemMode,
    #[strum(
        serialize = "/Game/GameData/Challenges/Character/Beastmaster/BP_Challenge_Beastmaster_ClassMod.BP_Challenge_Beastmaster_ClassMod_C",
        to_string = "BeastMaster Class Mod Slot"
    )]
    BeastMasterClassModSlot,
    #[strum(
        serialize = "/Game/GameData/Challenges/Character/Gunner/BP_Challenge_Gunner_ClassMod.BP_Challenge_Gunner_ClassMod_C",
        to_string = "Gunner Class Mod Slot"
    )]
    GunnerClassModSlot,
    #[strum(
        serialize = "/Game/GameData/Challenges/Character/Operative/BP_Challenge_Operative_ClassMod.BP_Challenge_Operative_ClassMod_C",
        to_string = "Operative Class Mod Slot"
    )]
    OperativeClassModSlot,
    #[strum(
        serialize = "/Game/GameData/Challenges/Character/Siren/BP_Challenge_Siren_ClassMod.BP_Challenge_Siren_ClassMod_C",
        to_string = "Siren Class Mod Slot"
    )]
    SirenClassModSlot,
}
