use crate::views::manage_save::character::CharacterMessage;
use crate::views::manage_save::general::GeneralMessage;
use crate::views::manage_save::main::{MainMessage, MainState, MainTabBarView};

pub mod character;
pub mod general;
pub mod main;

#[derive(Debug, Default)]
pub struct ManageSaveState {
    pub main_state: MainState,
}

#[derive(Debug, Clone)]
pub enum ManageSaveMessage {
    Main(MainMessage),
    General(GeneralMessage),
    Character(CharacterMessage),
}

#[derive(Debug, PartialEq)]
pub enum ManageSaveView {
    TabBar(MainTabBarView),
}
