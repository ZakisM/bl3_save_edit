use crate::views::manage_save::character::{CharacterInteractionMessage, CharacterMessage};
use crate::views::manage_save::general::GeneralInteractionMessage;
use crate::views::manage_save::main::{MainInteractionMessage, MainState, MainTabBarView};

pub mod character;
pub mod general;
pub mod main;

#[derive(Debug, Default)]
pub struct ManageSaveState {
    pub main_state: MainState,
}

#[derive(Debug)]
pub enum ManageSaveMessage {
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
