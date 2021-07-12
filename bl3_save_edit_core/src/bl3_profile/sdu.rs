use strum::{Display, EnumString};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct ProfSduSlotData {
    pub slot: ProfSduSlot,
    pub current: i32,
    pub max: i32,
}

#[derive(Debug, Display, EnumString, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum ProfSduSlot {
    #[strum(serialize = "/Game/Pickups/SDU/SDU_Bank.SDU_Bank", to_string = "Bank")]
    Bank,
    #[strum(
        serialize = "/Game/Pickups/SDU/SDU_LostLoot.SDU_LostLoot",
        to_string = "Lost Loot"
    )]
    LostLoot,
}

impl ProfSduSlot {
    pub fn maximum(&self) -> i32 {
        match *self {
            ProfSduSlot::Bank => 23,
            ProfSduSlot::LostLoot => 10,
        }
    }
}
