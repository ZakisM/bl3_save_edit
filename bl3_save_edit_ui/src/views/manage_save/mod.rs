use crate::views::manage_save::main::{MainMessage, MainState, MainTabBarView};

pub mod main;

#[derive(Debug, Default)]
pub struct ManageSaveState {
    main_state: MainState,
}

#[derive(Debug, Clone)]
pub enum ManageSaveMessage {
    Main(MainMessage),
}

#[derive(Debug, PartialEq)]
pub enum ManageSaveView {
    TabBar(MainTabBarView),
}
