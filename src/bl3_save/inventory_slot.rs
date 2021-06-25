use strum::{Display, EnumString};

#[derive(Debug)]
pub struct InventorySlotData {
    pub slot: InventorySlot,
    pub unlocked: bool,
}

#[derive(Debug, Display, EnumString, Eq, PartialEq)]
pub enum InventorySlot {
    #[strum(
        serialize = "/Game/Gear/Weapons/_Shared/_Design/InventorySlots/BPInvSlot_Weapon1.BPInvSlot_Weapon1",
        to_string = "Weapon 1"
    )]
    Weapon1,
    #[strum(
        serialize = "/Game/Gear/Weapons/_Shared/_Design/InventorySlots/BPInvSlot_Weapon2.BPInvSlot_Weapon2",
        to_string = "Weapon 2"
    )]
    Weapon2,
    #[strum(
        serialize = "/Game/Gear/Weapons/_Shared/_Design/InventorySlots/BPInvSlot_Weapon3.BPInvSlot_Weapon3",
        to_string = "Weapon 3"
    )]
    Weapon3,
    #[strum(
        serialize = "/Game/Gear/Weapons/_Shared/_Design/InventorySlots/BPInvSlot_Weapon4.BPInvSlot_Weapon4",
        to_string = "Weapon 4"
    )]
    Weapon4,
    #[strum(serialize = "/Game/Gear/Shields/_Design/A_Data/BPInvSlot_Shield.BPInvSlot_Shield", to_string = "Shield")]
    Shield,
    #[strum(
        serialize = "/Game/Gear/GrenadeMods/_Design/A_Data/BPInvSlot_GrenadeMod.BPInvSlot_GrenadeMod",
        to_string = "Grenade"
    )]
    Grenade,
    #[strum(
        serialize = "/Game/Gear/ClassMods/_Design/_Data/BPInvSlot_ClassMod.BPInvSlot_ClassMod",
        to_string = "Class Mod"
    )]
    ClassMod,
    #[strum(
        serialize = "/Game/Gear/Artifacts/_Design/_Data/BPInvSlot_Artifact.BPInvSlot_Artifact",
        to_string = "Artifact"
    )]
    Artifact,
}
