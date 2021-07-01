use strum::{Display, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ChallengeData {
    pub challenge: Challenge,
    pub unlocked: bool,
}

#[derive(Debug, Display, EnumString, Eq, PartialEq, Ord, PartialOrd)]
pub enum Challenge {
    #[strum(serialize = "Artifact Slot")]
    ArtifactSlot,
    #[strum(serialize = "Eridian Analyzer")]
    EridianAnalyzer,
    #[strum(serialize = "Eridian Resonator")]
    EridianResonator,
    #[strum(serialize = "Mayhem Mode")]
    MayhemMode,
    #[strum(serialize = "BeastMaster Class Mod Slot")]
    BeastMasterClassModSlot,
    #[strum(serialize = "Gunner Class Mod Slot")]
    GunnerClassModSlot,
    #[strum(serialize = "Operative Class Mod Slot")]
    OperativeClassModSlot,
    #[strum(serialize = "Siren Class Mod Slot")]
    SirenClassModSlot,
}
