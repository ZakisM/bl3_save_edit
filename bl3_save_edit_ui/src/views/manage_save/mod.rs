use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::manage_save::character::SaveCharacterInteractionMessage;
use crate::views::manage_save::currency::SaveCurrencyInteractionMessage;
use crate::views::manage_save::general::SaveGeneralInteractionMessage;
use crate::views::manage_save::inventory::SaveInventoryInteractionMessage;
use crate::views::manage_save::main::{
    SaveTabBarInteractionMessage, SaveTabBarView, SaveViewState,
};
use crate::views::manage_save::vehicle::SaveVehicleInteractionMessage;

pub mod character;
pub mod currency;
pub mod general;
pub mod inventory;
pub mod main;
pub mod vehicle;

#[derive(Debug, Default)]
pub struct ManageSaveState {
    pub save_view_state: SaveViewState,
    pub current_file: Bl3Save,
}

#[derive(Debug, Clone)]
pub enum ManageSaveInteractionMessage {
    TabBar(SaveTabBarInteractionMessage),
    General(SaveGeneralInteractionMessage),
    Character(SaveCharacterInteractionMessage),
    Inventory(SaveInventoryInteractionMessage),
    Currency(SaveCurrencyInteractionMessage),
    Vehicle(SaveVehicleInteractionMessage),
    SaveFilePressed,
}

#[derive(Debug, PartialEq)]
pub enum ManageSaveView {
    TabBar(SaveTabBarView),
}
