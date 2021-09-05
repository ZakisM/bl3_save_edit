use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::manage_save::character::SaveCharacterInteractionMessage;
use crate::views::manage_save::currency::SaveCurrencyInteractionMessage;
use crate::views::manage_save::general::{GeneralMessage, SaveGeneralInteractionMessage};
use crate::views::manage_save::inventory::SaveInventoryInteractionMessage;
use crate::views::manage_save::main::{
    SaveTabBarInteractionMessage, SaveTabBarView, SaveViewState,
};

pub mod character;
pub mod currency;
pub mod general;
pub mod inventory;
pub mod main;

#[derive(Debug, Default)]
pub struct ManageSaveState {
    pub save_view_state: SaveViewState,
    pub current_file: Bl3Save,
}

//These messages are currently only being used for async messages
#[derive(Debug, Clone)]
pub enum ManageSaveMessage {
    General(GeneralMessage),
}

#[derive(Debug, Clone)]
pub enum ManageSaveInteractionMessage {
    TabBar(SaveTabBarInteractionMessage),
    General(SaveGeneralInteractionMessage),
    Character(SaveCharacterInteractionMessage),
    Inventory(SaveInventoryInteractionMessage),
    Currency(SaveCurrencyInteractionMessage),
    SaveFilePressed,
}

#[derive(Debug, PartialEq)]
pub enum ManageSaveView {
    TabBar(SaveTabBarView),
}
