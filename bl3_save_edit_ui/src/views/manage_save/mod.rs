use bl3_save_edit_core::bl3_save::Bl3Save;

use crate::views::manage_save::character::{CharacterInteractionMessage, CharacterMessage};
use crate::views::manage_save::general::{GeneralInteractionMessage, GeneralMessage};
use crate::views::manage_save::main::{MainInteractionMessage, MainState, MainTabBarView};

pub mod character;
pub mod general;
pub mod main;

#[derive(Debug, Default)]
pub struct ManageSaveState {
    pub main_state: MainState,
    pub current_file: Bl3Save,
}

#[derive(Debug)]
pub enum ManageSaveMessage {
    General(GeneralMessage),
    Character(CharacterMessage),
}

#[derive(Debug, Clone)]
pub enum ManageSaveInteractionMessage {
    Main(MainInteractionMessage),
    General(GeneralInteractionMessage),
    Character(CharacterInteractionMessage),
}

#[derive(Debug, PartialEq)]
pub enum ManageSaveView {
    TabBar(MainTabBarView),
}
