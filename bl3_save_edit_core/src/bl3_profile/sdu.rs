use strum::{Display, EnumIter, EnumMessage, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct ProfileSduSlotData {
    pub slot: ProfileSduSlot,
    pub current: i32,
    pub max: i32,
}

#[derive(
    Debug, Display, EnumString, EnumIter, EnumMessage, Eq, PartialEq, Ord, PartialOrd, Clone,
)]
pub enum ProfileSduSlot {
    #[strum(serialize = "/Game/Pickups/SDU/SDU_Bank.SDU_Bank", to_string = "Bank")]
    Bank,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_LostLoot.SDU_LostLoot",
        to_string = "Lost Loot"
    )]
    LostLoot,
}

impl ProfileSduSlot {
    pub fn maximum(&self) -> i32 {
        match self {
            ProfileSduSlot::Bank => 23,
            ProfileSduSlot::LostLoot => 10,
        }
    }
}

impl std::default::Default for ProfileSduSlot {
    fn default() -> Self {
        Self::Bank
    }
}
